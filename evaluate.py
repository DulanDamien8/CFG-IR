#!/usr/bin/env python3
"""
CFG-Based Metamorphic Engine Evaluation Script
Measures the effectiveness of CFG-based metamorphic transformations using:
1. Opcode n-gram similarity
2. Fuzzy hash similarity (ssdeep)
3. Control-Flow Graph (CFG) similarity
"""

import os
import sys
import json
import networkx as nx
from collections import Counter
from pathlib import Path
import matplotlib.pyplot as plt
import numpy as np

try:
    import ssdeep
    SSDEEP_AVAILABLE = True
except ImportError:
    SSDEEP_AVAILABLE = False
    print("[!] ssdeep not available — fuzzy hash metric will be skipped.")


class TeeLogger:
    """Duplicates stdout writes to a log file — captures everything printed to the terminal."""
    def __init__(self, log_path):
        self.terminal = sys.stdout
        self.log = open(log_path, 'w', encoding='utf-8')

    def write(self, message):
        self.terminal.write(message)
        self.log.write(message)
        self.log.flush()

    def flush(self):
        self.terminal.flush()
        self.log.flush()

    def close(self):
        self.log.close()


class CFGMetamorphicEvaluator:
    def __init__(self, output_dir="output"):
        self.output_dir = Path(output_dir)
        self.results = {}

    def read_opcodes(self, filename):
        with open(self.output_dir / filename, 'r') as f:
            return [line.strip() for line in f if line.strip()]

    def read_assembly(self, filename):
        with open(self.output_dir / filename, 'r') as f:
            return f.read()

    # ==================== METRIC 1: Opcode N-Gram Similarity ====================

    def generate_ngrams(self, opcodes, n=3):
        if len(opcodes) < n:
            return []
        return [tuple(opcodes[i:i+n]) for i in range(len(opcodes) - n + 1)]

    def cosine_similarity(self, vec1, vec2):
        intersection = set(vec1.keys()) & set(vec2.keys())
        numerator = sum([vec1[x] * vec2[x] for x in intersection])
        sum1 = sum([vec1[x]**2 for x in vec1.keys()])
        sum2 = sum([vec2[x]**2 for x in vec2.keys()])
        denominator = (sum1 ** 0.5) * (sum2 ** 0.5)
        if denominator == 0:
            return 0.0
        return numerator / denominator

    def jaccard_similarity(self, set1, set2):
        intersection = len(set1.intersection(set2))
        union = len(set1.union(set2))
        if union == 0:
            return 0.0
        return intersection / union

    def calculate_ngram_similarity(self, opcodes1, opcodes2, n=3):
        ngrams1 = self.generate_ngrams(opcodes1, n)
        ngrams2 = self.generate_ngrams(opcodes2, n)
        counter1 = Counter(ngrams1)
        counter2 = Counter(ngrams2)
        cosine_sim = self.cosine_similarity(counter1, counter2)
        jaccard_sim = self.jaccard_similarity(set(ngrams1), set(ngrams2))
        return {'cosine': cosine_sim, 'jaccard': jaccard_sim, 'n': n}

    def evaluate_ngram_similarity(self):
        """Evaluate n-gram similarity across all variants"""
        print("\n" + "="*70)
        print("METRIC 1: OPCODE N-GRAM SIMILARITY (CFG Engine)")
        print("="*70)

        original_opcodes = self.read_opcodes("cfg_original_opcodes.txt")
        results = {'original': original_opcodes, 'variants': {}, 'similarities': []}
        n_values = [3, 4, 5]

        variant_files = sorted([f for f in os.listdir(self.output_dir)
                               if f.startswith('cfg_variant_') and f.endswith('_opcodes.txt')])

        for vf in variant_files:
            vname = vf.replace('_opcodes.txt', '')
            vopcodes = self.read_opcodes(vf)
            results['variants'][vname] = vopcodes

            print(f"\n{vname}:")
            print(f"  Original length: {len(original_opcodes)} instructions")
            print(f"  Variant length:  {len(vopcodes)} instructions")
            print(f"  Expansion rate:  {len(vopcodes)/max(len(original_opcodes),1):.2f}x")

            for n in n_values:
                sim = self.calculate_ngram_similarity(original_opcodes, vopcodes, n)
                results['similarities'].append({'variant': vname, 'n': n, **sim})
                print(f"  {n}-gram similarity:")
                print(f"    Cosine:  {sim['cosine']:.4f} (lower is better)")
                print(f"    Jaccard: {sim['jaccard']:.4f} (lower is better)")

        # Calculate average similarities
        print("\n" + "-"*70)
        print("AVERAGE SIMILARITY ACROSS ALL VARIANTS:")
        print("-"*70)
        for n in n_values:
            nr = [s for s in results['similarities'] if s['n'] == n]
            avg_cosine = np.mean([s['cosine'] for s in nr])
            avg_jaccard = np.mean([s['jaccard'] for s in nr])
            print(f"{n}-gram:")
            print(f"  Average Cosine:  {avg_cosine:.4f}")
            print(f"  Average Jaccard: {avg_jaccard:.4f}")

        self.results['ngram'] = results

    # ==================== METRIC 2: Fuzzy Hash Similarity ====================

    def evaluate_fuzzy_hash_similarity(self):
        """Evaluate fuzzy hash similarity using ssdeep on .asm source and .exe binaries"""
        if not SSDEEP_AVAILABLE:
            print("\n" + "="*70)
            print("METRIC 2: FUZZY HASH SIMILARITY (ssdeep) — SKIPPED (not installed)")
            print("="*70)
            return

        print("\n" + "="*70)
        print("METRIC 2: FUZZY HASH SIMILARITY (ssdeep) (CFG Engine)")
        print("="*70)

        original_asm = self.read_assembly("cfg_original.asm").encode('utf-8')
        original_hash = ssdeep.hash(original_asm)
        print(f"\nOriginal ssdeep hash (asm): {original_hash}")

        results = {'original_hash': original_hash, 'variants': {}, 'similarities': []}
        variant_files = sorted([f for f in os.listdir(self.output_dir)
                               if f.startswith('cfg_variant_') and f.endswith('.asm')])

        for vf in variant_files:
            vname = vf.replace('.asm', '')
            vasm = self.read_assembly(vf).encode('utf-8')
            vhash = ssdeep.hash(vasm)
            sim = ssdeep.compare(original_hash, vhash)
            results['variants'][vname] = {'hash': vhash, 'similarity': sim}
            results['similarities'].append(sim)

            print(f"\n{vname}:")
            print(f"  ssdeep hash (asm): {vhash}")
            print(f"  Similarity (asm):  {sim}% (lower is better)")

        # Binary comparison if .exe files exist
        original_exe = self.output_dir / "cfg_original.exe"
        if original_exe.exists():
            print(f"\n{'—'*70}")
            print("Binary-level fuzzy hash comparison (.exe):")
            obin_hash = ssdeep.hash(original_exe.read_bytes())
            print(f"  Original exe hash: {obin_hash}")

            results['binary'] = {'original_hash': obin_hash, 'variants': {}, 'similarities': []}
            for vf in variant_files:
                vname = vf.replace('.asm', '')
                exe_path = self.output_dir / f"{vname}.exe"
                if exe_path.exists():
                    vbin_hash = ssdeep.hash(exe_path.read_bytes())
                    bsim = ssdeep.compare(obin_hash, vbin_hash)
                    results['binary']['variants'][vname] = {'hash': vbin_hash, 'similarity': bsim}
                    results['binary']['similarities'].append(bsim)
                    print(f"  {vname} exe similarity: {bsim}%")
            if results['binary']['similarities']:
                avg_bin = np.mean(results['binary']['similarities'])
                print(f"  Average binary similarity: {avg_bin:.2f}%")

        avg_similarity = np.mean(results['similarities']) if results['similarities'] else 0
        print(f"\n{'='*70}")
        print(f"AVERAGE FUZZY HASH SIMILARITY (asm): {avg_similarity:.2f}%")
        print(f"{'='*70}")

        self.results['fuzzy_hash'] = results

    # ==================== METRIC 3: CFG Similarity ====================

    def build_cfg(self, opcodes):
        G = nx.DiGraph()
        for i, opcode in enumerate(opcodes):
            G.add_node(i, opcode=opcode)
            if opcode not in ['call', 'ret', 'jmp', 'je', 'jne', 'jz', 'jnz']:
                if i + 1 < len(opcodes):
                    G.add_edge(i, i + 1)
            elif opcode == 'call':
                if i + 1 < len(opcodes):
                    G.add_edge(i, i + 1)
        return G

    def calculate_graph_edit_distance(self, G1, G2):
        nodes1, nodes2 = set(G1.nodes()), set(G2.nodes())
        edges1, edges2 = set(G1.edges()), set(G2.edges())
        total = len(nodes1 - nodes2) + len(nodes2 - nodes1) + len(edges1 - edges2) + len(edges2 - edges1)
        max_size = max(len(nodes1) + len(edges1), len(nodes2) + len(edges2))
        return total / max_size if max_size > 0 else 0.0

    def calculate_subgraph_similarity(self, G1, G2):
        ds1 = sorted([d for _, d in G1.degree()], reverse=True)
        ds2 = sorted([d for _, d in G2.degree()], reverse=True)
        min_len, max_len = min(len(ds1), len(ds2)), max(len(ds1), len(ds2))
        if max_len == 0:
            return 1.0
        return sum(1 for i in range(min_len) if ds1[i] == ds2[i]) / max_len

    def evaluate_cfg_similarity(self):
        """Evaluate Control-Flow Graph similarity"""
        print("\n" + "="*70)
        print("METRIC 3: CONTROL-FLOW GRAPH (CFG) SIMILARITY (CFG Engine)")
        print("="*70)

        original_opcodes = self.read_opcodes("cfg_original_opcodes.txt")
        original_cfg = self.build_cfg(original_opcodes)

        print(f"\nOriginal CFG:")
        print(f"  Nodes: {original_cfg.number_of_nodes()}")
        print(f"  Edges: {original_cfg.number_of_edges()}")

        results = {'original_cfg': {'nodes': original_cfg.number_of_nodes(), 'edges': original_cfg.number_of_edges()},
                   'variants': {}, 'similarities': {'ged': [], 'subgraph': []}}

        variant_files = sorted([f for f in os.listdir(self.output_dir)
                               if f.startswith('cfg_variant_') and f.endswith('_opcodes.txt')])

        for vf in variant_files:
            vname = vf.replace('_opcodes.txt', '')
            vopcodes = self.read_opcodes(vf)
            vcfg = self.build_cfg(vopcodes)
            ged = self.calculate_graph_edit_distance(original_cfg, vcfg)
            sub = self.calculate_subgraph_similarity(original_cfg, vcfg)
            results['variants'][vname] = {'nodes': vcfg.number_of_nodes(), 'edges': vcfg.number_of_edges(), 'ged': ged, 'subgraph_similarity': sub}
            results['similarities']['ged'].append(ged)
            results['similarities']['subgraph'].append(sub)

            print(f"\n{vname}:")
            print(f"  Nodes: {vcfg.number_of_nodes()}")
            print(f"  Edges: {vcfg.number_of_edges()}")
            print(f"  Graph Edit Distance (normalized): {ged:.4f} (higher is better)")
            print(f"  Subgraph Similarity: {sub:.4f} (lower is better)")

        avg_ged = np.mean(results['similarities']['ged'])
        avg_subgraph = np.mean(results['similarities']['subgraph'])

        print(f"\n{'='*70}")
        print(f"AVERAGE CFG METRICS:")
        print(f"  Graph Edit Distance: {avg_ged:.4f} (higher is better)")
        print(f"  Subgraph Similarity: {avg_subgraph:.4f} (lower is better)")
        print(f"{'='*70}")

        self.results['cfg'] = results

    # ==================== VISUALIZATION ====================

    def generate_visualizations(self):
        print("\n" + "="*70)
        print("GENERATING VISUALIZATIONS")
        print("="*70)

        fig, axes = plt.subplots(2, 2, figsize=(15, 12))
        fig.suptitle('CFG-Based Metamorphic Transformation Effectiveness', fontsize=16, fontweight='bold')

        if 'ngram' in self.results:
            ax = axes[0, 0]
            data = self.results['ngram']['similarities']
            variants = sorted(set([d['variant'] for d in data]))
            n_values = sorted(set([d['n'] for d in data]))
            x = np.arange(len(variants))
            width = 0.25
            for i, n in enumerate(n_values):
                nd = [d for d in data if d['n'] == n]
                ax.bar(x + i*width, [d['cosine'] for d in nd], width, label=f'{n}-gram')
            ax.set_xlabel('Variant')
            ax.set_ylabel('Cosine Similarity')
            ax.set_title('Opcode N-gram Similarity (Lower = Better)')
            ax.set_xticks(x + width)
            ax.set_xticklabels([v.replace('cfg_variant_', 'V') for v in variants], rotation=45, fontsize=6)
            ax.legend()
            ax.grid(True, alpha=0.3)

        if 'fuzzy_hash' in self.results:
            ax = axes[0, 1]
            variants = sorted(self.results['fuzzy_hash']['variants'].keys())
            sims = [self.results['fuzzy_hash']['variants'][v]['similarity'] for v in variants]
            ax.bar(range(len(variants)), sims, color='coral')
            ax.set_xlabel('Variant')
            ax.set_ylabel('Similarity (%)')
            ax.set_title('Fuzzy Hash Similarity (Lower = Better)')
            ax.set_xticks(range(len(variants)))
            ax.set_xticklabels([v.replace('cfg_variant_', 'V') for v in variants], rotation=45, fontsize=6)
            ax.grid(True, alpha=0.3)

        if 'cfg' in self.results:
            ax = axes[1, 0]
            variants = sorted(self.results['cfg']['variants'].keys())
            geds = [self.results['cfg']['variants'][v]['ged'] for v in variants]
            ax.bar(range(len(variants)), geds, color='lightgreen')
            ax.set_xlabel('Variant')
            ax.set_ylabel('Normalized GED')
            ax.set_title('CFG Graph Edit Distance (Higher = Better)')
            ax.set_xticks(range(len(variants)))
            ax.set_xticklabels([v.replace('cfg_variant_', 'V') for v in variants], rotation=45, fontsize=6)
            ax.grid(True, alpha=0.3)

        if 'ngram' in self.results:
            ax = axes[1, 1]
            orig_len = len(self.results['ngram']['original'])
            variants = sorted(self.results['ngram']['variants'].keys())
            rates = [len(self.results['ngram']['variants'][v]) / max(orig_len, 1) for v in variants]
            ax.bar(range(len(variants)), rates, color='skyblue')
            ax.set_xlabel('Variant')
            ax.set_ylabel('Expansion Rate')
            ax.set_title('Code Expansion Rate')
            ax.set_xticks(range(len(variants)))
            ax.set_xticklabels([v.replace('cfg_variant_', 'V') for v in variants], rotation=45, fontsize=6)
            ax.axhline(y=1.0, color='r', linestyle='--', label='Original size')
            ax.legend()
            ax.grid(True, alpha=0.3)

        plt.tight_layout()
        plt.savefig(self.output_dir / 'cfg_evaluation_results.png', dpi=300, bbox_inches='tight')
        print(f"[+] Visualization saved to: {self.output_dir / 'cfg_evaluation_results.png'}")

    # ==================== REPORTING ====================

    def save_json_results(self):
        """Save the full results dictionary as JSON (matches Assembly evaluator structure)."""
        json_path = self.output_dir / 'cfg_evaluation_results.json'
        with open(json_path, 'w') as f:
            json_results = json.loads(json.dumps(self.results, default=lambda x: float(x) if isinstance(x, np.number) else x))
            json.dump(json_results, f, indent=2)
        print(f"\n[+] JSON results saved to: {json_path}")

    def run_full_evaluation(self):
        """Run all evaluation metrics with full terminal output captured to report file."""
        # Set up tee logger — all print() output goes to both terminal AND report file
        self.output_dir.mkdir(parents=True, exist_ok=True)
        report_path = self.output_dir / 'cfg_evaluation_report.txt'
        tee = TeeLogger(report_path)
        sys.stdout = tee

        try:
            print("\n" + "="*70)
            print("STARTING CFG-BASED METAMORPHIC ENGINE EVALUATION")
            print("="*70)
            self.evaluate_ngram_similarity()
            self.evaluate_fuzzy_hash_similarity()
            self.evaluate_cfg_similarity()
            self.generate_visualizations()
            self.save_json_results()
            print("\n" + "="*70)
            print("EVALUATION COMPLETE")
            print("="*70)
            print(f"\n[+] Full terminal output saved to: {report_path}")
        finally:
            sys.stdout = tee.terminal
            tee.close()


if __name__ == "__main__":
    CFGMetamorphicEvaluator("output").run_full_evaluation()