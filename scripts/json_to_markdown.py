#!/usr/bin/env python3
import json
import sys
from collections import defaultdict

def format_time(ns):
    """Convert nanoseconds to human readable format"""
    if ns < 1000:
        return f"{ns:.0f} ns"
    elif ns < 1000000:
        return f"{ns/1000:.1f} μs"
    elif ns < 1000000000:
        return f"{ns/1000000:.1f} ms"
    else:
        return f"{ns/1000000000:.2f} s"

def parse_benchmark_results(filename):
    """Parse criterion JSON output and organize by operation and library"""
    # Structure: {operation: {library: {bit_size: time_ns}}}
    results = defaultdict(lambda: defaultdict(dict))
    
    with open(filename, 'r') as f:
        for line in f:
            try:
                data = json.loads(line)
                if data.get('reason') == 'benchmark-complete':
                    bench_id = data['id']
                    estimate = data['typical']['estimate']
                    
                    # Parse benchmark name
                    parts = bench_id.split('/')
                    if len(parts) >= 3:
                        # Expected format: "Library Addition/library-name/64" or "Library Multiplication/library-name/64"
                        group = parts[0]  # e.g., "Nail Addition"
                        library = parts[1]  # e.g., "nail"
                        bit_size = parts[2]  # e.g., "64"
                        
                        # Determine operation type from group name
                        if 'Addition' in group:
                            operation = 'Addition'
                        elif 'Multiplication' in group:
                            operation = 'Multiplication'
                        elif 'Modular' in group:
                            # Skip modular operations for now
                            continue
                        else:
                            continue
                            
                        results[operation][library][bit_size] = estimate
                        
            except Exception as e:
                continue
    
    return results

def generate_comparison_tables(results):
    """Generate comparison tables for each operation"""
    markdown = "# Benchmark Results\n\n"
    markdown += "*Generated automatically from criterion benchmark results*\n\n"
    
    for operation, libraries in results.items():
        markdown += f"## {operation} Performance\n\n"
        
        # Get all bit sizes and sort them
        all_bit_sizes = set()
        for lib_data in libraries.values():
            all_bit_sizes.update(lib_data.keys())
        bit_sizes = sorted(all_bit_sizes, key=lambda x: int(x) if x.isdigit() else float('inf'))
        
        # Get all libraries
        library_names = sorted(libraries.keys())
        
        if not bit_sizes or not library_names:
            continue
            
        # Create table header
        markdown += "| Library |"
        for bit_size in bit_sizes:
            markdown += f" {bit_size}-bit |"
        markdown += "\n"
        
        # Create separator
        markdown += "|---------|"
        for _ in bit_sizes:
            markdown += "---------|"
        markdown += "\n"
        
        # Create rows for each library
        for library in library_names:
            lib_data = libraries[library]
            markdown += f"| **{library}** |"
            
            for bit_size in bit_sizes:
                if bit_size in lib_data:
                    time_formatted = format_time(lib_data[bit_size])
                    markdown += f" {time_formatted} |"
                else:
                    markdown += " - |"
            markdown += "\n"
        
        markdown += "\n"
        
        # Add performance summary
        markdown += f"### {operation} Performance Summary\n\n"
        for bit_size in bit_sizes:
            fastest_lib = None
            fastest_time = float('inf')
            
            for library, lib_data in libraries.items():
                if bit_size in lib_data and lib_data[bit_size] < fastest_time:
                    fastest_time = lib_data[bit_size]
                    fastest_lib = library
            
            if fastest_lib:
                markdown += f"- **{bit_size}-bit**: Fastest is **{fastest_lib}** ({format_time(fastest_time)})\n"
        
        markdown += "\n"
    
    return markdown

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python json_to_markdown.py <benchmark_results.json>")
        sys.exit(1)
    
    filename = sys.argv[1]
    results = parse_benchmark_results(filename)
    markdown = generate_comparison_tables(results)
    print(markdown)