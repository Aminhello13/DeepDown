import { useState } from "react";

interface SearchResult {
  name: string;
  url: string;
  size: string | null;
  source: string;
}

export default function App() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState<string | null>(null);

  async function handleSearch() {
    if (!query.trim()) return;
    setLoading(true);
    setResults([]);
    setProgress("Searching across registry...");

    try {
      const res = await fetch(
        `http://localhost:9090/search?q=${encodeURIComponent(query)}`
      );
      const data = await res.json();
      setResults(data.results || []);
      setProgress(null);
    } catch {
      // Fallback demo mode — no core engine running
      setProgress(null);
      setResults([
        {
          name: "Kali Linux 2024.4 Installer (amd64)",
          url: "https://cdimage.kali.org/kali-latest/kali-linux-2024.4-installer-amd64.iso",
          size: "4.2 GB",
          source: "cdimage.kali.org",
        },
        {
          name: "Kali Linux 2024.4 Installer (arm64)",
          url: "https://cdimage.kali.org/kali-latest/kali-linux-2024.4-installer-arm64.iso",
          size: "3.8 GB",
          source: "cdimage.kali.org",
        },
        {
          name: "Ubuntu 24.04.1 Desktop (amd64)",
          url: "https://releases.ubuntu.com/24.04/ubuntu-24.04.1-desktop-amd64.iso",
          size: "5.9 GB",
          source: "releases.ubuntu.com",
        },
      ]);
    }
    setLoading(false);
  }

  async function handleDownload(url: string, name: string) {
    setProgress(`Downloading: ${name}...`);
    try {
      await fetch(`http://localhost:9090/download?url=${encodeURIComponent(url)}`);
    } catch {
      // Demo mode — just open in browser
      window.open(url, "_blank");
    }
    setProgress(null);
  }

  return (
    <div className="min-h-screen bg-grayx-900 text-grayx-100">
      {/* Header */}
      <header className="border-b border-grayx-700 px-6 py-4">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-deepx-500 flex items-center justify-center text-white font-bold text-xl">
            D
          </div>
          <div>
            <h1 className="text-xl font-bold text-deepx-300">DeepDown</h1>
            <p className="text-xs text-grayx-400">Universal Download Source Aggregator</p>
          </div>
        </div>
      </header>

      {/* Search */}
      <main className="max-w-4xl mx-auto p-6">
        <div className="flex gap-2 mb-6">
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSearch()}
            placeholder="Search for anything... (e.g. Kali Linux latest)"
            className="flex-1 bg-grayx-800 border border-grayx-700 rounded-lg px-4 py-3 text-grayx-100 placeholder-grayx-500 focus:outline-none focus:border-deepx-500 transition-colors"
          />
          <button
            onClick={handleSearch}
            disabled={loading}
            className="bg-deepx-500 hover:bg-deepx-600 disabled:opacity-50 text-white font-medium px-6 py-3 rounded-lg transition-colors"
          >
            {loading ? "Searching..." : "Search"}
          </button>
        </div>

        {/* Progress */}
        {progress && (
          <div className="bg-deepx-900/50 border border-deepx-700 text-deepx-200 rounded-lg p-4 mb-6">
            {progress}
          </div>
        )}

        {/* Results */}
        <div className="space-y-3">
          {results.map((result, idx) => (
            <div
              key={idx}
              className="bg-grayx-800 border border-grayx-700 hover:border-deepx-500 rounded-lg p-4 transition-colors"
            >
              <div className="flex items-start justify-between gap-4">
                <div className="flex-1 min-w-0">
                  <h3 className="text-grayx-100 font-medium truncate">{result.name}</h3>
                  <p className="text-sm text-grayx-400 mt-1">
                    <span className="text-deepx-300">{result.source}</span>
                    {result.size && <span> - {result.size}</span>}
                  </p>
                  <p className="text-xs text-grayx-500 mt-1 truncate font-mono">{result.url}</p>
                </div>
                <button
                  onClick={() => handleDownload(result.url, result.name)}
                  className="shrink-0 bg-deepx-500 hover:bg-deepx-600 text-white text-sm font-medium px-4 py-2 rounded-lg transition-colors"
                >
                  Download
                </button>
              </div>
            </div>
          ))}
        </div>

        {results.length === 0 && !loading && !progress && (
          <div className="text-center text-grayx-500 py-20">
            <p className="text-lg">Search across hundreds of sources</p>
            <p className="text-sm mt-2">Direct links. No ads. No clutter.</p>
          </div>
        )}

        {/* Footer */}
        <footer className="mt-16 border-t border-grayx-700 pt-4 text-center text-xs text-grayx-500">
          DeepDown v0.1 - DarXone Group
        </footer>
      </main>
    </div>
  );
}
