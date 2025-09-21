#!/usr/bin/env python3
import json
import sys
from collections import defaultdict

def format_time(ns):
    """Convert nanoseconds to human readable format"""
    if ns < 1000:
        return f"{ns:.2f} ns"
    elif ns < 1000000:
        return f"{ns/1000:.2f} μs"
    elif ns < 1000000000:
        return f"{ns/1000000:.2f} ms"
    else:
        return f"{ns/1000000000:.2f} s"

def parse_benchmark_results(filename):
    """Parse criterion JSON output and group by benchmark types"""
    results = defaultdict(list)
    
    with open(filename, 'r') as f:
        for line in f:
            try:
                data = json.loads(line)
                if data.get('reason') == 'benchmark-complete':
                    bench_id = data['id']
                    estimate = data['typical']['estimate']
                    
                    # Parse benchmark name and extract info
                    parts = bench_id.split('/')
                    if len(parts) >= 2:
                        group = parts[0]
                        library = parts[1]
                        param = parts[2] if len(parts) > 2 else ""
                        
                        results[group].append({
                            'library': library,
                            'parameter': param,
                            'time_ns': estimate,
                            'time_formatted': format_time(estimate)
                        })
            except:
                continue
    
    return results

def generate_markdown_table(results):
    """Generate markdown tables for each benchmark group"""
    markdown = "# Benchmark Results\n\n"
    markdown += "*Generated automatically from criterion benchmark results*\n\n"
    
    for group, benchmarks in results.items():
        markdown += f"## {group}\n\n"
        
        # Sort by time
        benchmarks.sort(key=lambda x: x['time_ns'])
        
        # Create table
        markdown += "| Library | Parameter | Time |\n"
        markdown += "|---------|-----------|------|\n"
        
        for bench in benchmarks:
            param = bench['parameter'] or '-'
            markdown += f"| {bench['library']} | {param} | {bench['time_formatted']} |\n"
        
        markdown += "\n"
    
    return markdown

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python json_to_markdown.py <benchmark_results.json>")
        sys.exit(1)
    
    filename = sys.argv[1]
    results = parse_benchmark_results(filename)
    markdown = generate_markdown_table(results)
    print(markdown)