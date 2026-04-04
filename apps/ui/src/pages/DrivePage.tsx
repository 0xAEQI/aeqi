import { useState, useEffect, useCallback } from "react";
import { useNavigate } from "react-router-dom";
import { useAuthStore } from "@/store/auth";

interface VfsNode {
  name: string;
  path: string;
  node_type: "directory" | "file" | "database";
  size?: number;
  modified?: string;
  mime?: string;
  icon?: string;
  badge?: string;
}

interface VfsListResponse {
  path: string;
  nodes: VfsNode[];
}

interface VfsReadResponse {
  path: string;
  content: string;
  mime: string;
  editable: boolean;
}

function NodeIcon({ node }: { node: VfsNode }) {
  if (node.icon) {
    return <span className="vfs-icon">{node.icon}</span>;
  }
  if (node.node_type === "directory") {
    return <span className="vfs-icon">📁</span>;
  }
  if (node.mime?.includes("json")) {
    return <span className="vfs-icon">{ }</span>;
  }
  if (node.mime?.includes("markdown")) {
    return <span className="vfs-icon">📝</span>;
  }
  return <span className="vfs-icon">📄</span>;
}

function Badge({ label }: { label: string }) {
  const cls = `vfs-badge vfs-badge-${label}`;
  return <span className={cls}>{label}</span>;
}

function Breadcrumb({ path, onNavigate }: { path: string; onNavigate: (p: string) => void }) {
  const parts = path.split("/").filter(Boolean);
  const segments = [{ label: "drive", path: "/" }];
  let acc = "";
  for (const p of parts) {
    acc += `/${p}`;
    segments.push({ label: p, path: acc });
  }

  return (
    <div className="vfs-breadcrumb">
      {segments.map((seg, i) => (
        <span key={seg.path}>
          {i > 0 && <span className="vfs-breadcrumb-sep">/</span>}
          <button
            className="vfs-breadcrumb-item"
            onClick={() => onNavigate(seg.path)}
          >
            {seg.label}
          </button>
        </span>
      ))}
    </div>
  );
}

function FileContent({ response }: { response: VfsReadResponse }) {
  const isJson = response.mime.includes("json");
  const isMarkdown = response.mime.includes("markdown");

  if (isJson) {
    try {
      const parsed = JSON.parse(response.content);
      return (
        <pre className="vfs-file-content vfs-json">
          {JSON.stringify(parsed, null, 2)}
        </pre>
      );
    } catch {
      return <pre className="vfs-file-content">{response.content}</pre>;
    }
  }

  if (isMarkdown) {
    // Simple markdown rendering
    const lines = response.content.split("\n");
    return (
      <div className="vfs-file-content vfs-markdown">
        {lines.map((line, i) => {
          if (line.startsWith("# ")) return <h1 key={i}>{line.slice(2)}</h1>;
          if (line.startsWith("## ")) return <h2 key={i}>{line.slice(3)}</h2>;
          if (line.startsWith("### ")) return <h3 key={i}>{line.slice(4)}</h3>;
          if (line.startsWith("- ")) return <li key={i}>{line.slice(2)}</li>;
          if (line.startsWith("**") && line.endsWith("**")) {
            return <p key={i}><strong>{line.slice(2, -2)}</strong></p>;
          }
          if (line.startsWith("---")) return <hr key={i} />;
          if (line.trim() === "") return <br key={i} />;
          return <p key={i}>{line}</p>;
        })}
      </div>
    );
  }

  return <pre className="vfs-file-content">{response.content}</pre>;
}

export default function DrivePage() {
  const navigate = useNavigate();
  const token = useAuthStore((s) => s.token);
  const [currentPath, setCurrentPath] = useState("/");
  const [nodes, setNodes] = useState<VfsNode[]>([]);
  const [selectedFile, setSelectedFile] = useState<VfsReadResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState("");
  const [searchResults, setSearchResults] = useState<VfsNode[]>([]);
  const [isSearching, setIsSearching] = useState(false);

  const fetchList = useCallback(async (path: string) => {
    if (!token) return;
    setLoading(true);
    setError(null);
    setSelectedFile(null);

    try {
      const res = await fetch(`/api/vfs?path=${encodeURIComponent(path)}`, {
        headers: { Authorization: `Bearer ${token}` },
      });
      const data = await res.json();
      if (data.nodes) {
        setNodes(data.nodes);
        setCurrentPath(path);
      } else {
        setError(data.error || "Failed to list directory");
      }
    } catch (e: any) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  }, [token]);

  const fetchRead = useCallback(async (path: string) => {
    if (!token) return;
    setLoading(true);
    setError(null);

    try {
      const res = await fetch(`/api/vfs/${path}`, {
        headers: { Authorization: `Bearer ${token}` },
      });
      const data = await res.json();
      if (data.content !== undefined) {
        setSelectedFile(data);
      } else {
        setError(data.error || "Failed to read file");
      }
    } catch (e: any) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  }, [token]);

  const handleSearch = useCallback(async () => {
    if (!token || !searchQuery.trim()) return;
    setIsSearching(true);
    setError(null);

    try {
      const res = await fetch(`/api/vfs/search?query=${encodeURIComponent(searchQuery)}`, {
        headers: { Authorization: `Bearer ${token}` },
      });
      const data = await res.json();
      if (data.results) {
        setSearchResults(data.results.map((r: any) => ({
          name: r.name,
          path: r.path,
          node_type: r.node_type,
          icon: r.snippet ? "🔍" : undefined,
        })));
      }
    } catch (e: any) {
      setError(e.message);
    } finally {
      setIsSearching(false);
    }
  }, [token, searchQuery]);

  useEffect(() => {
    fetchList("/");
  }, [fetchList]);

  const handleNodeClick = (node: VfsNode) => {
    if (node.node_type === "directory") {
      fetchList(node.path);
    } else {
      fetchRead(node.path);
    }
  };

  const handleNavigate = (path: string) => {
    fetchList(path);
  };

  const handleBack = () => {
    const parts = currentPath.split("/").filter(Boolean);
    parts.pop();
    const parent = parts.length > 0 ? `/${parts.join("/")}` : "/";
    fetchList(parent);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Backspace" && !searchQuery) {
      handleBack();
    }
    if (e.key === "Enter" && searchQuery) {
      handleSearch();
    }
  };

  return (
    <div className="drive-page" onKeyDown={handleKeyDown} tabIndex={0}>
      <div className="drive-header">
        <h1>Drive</h1>
        <div className="drive-search">
          <input
            type="text"
            placeholder="Search across all nodes..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSearch()}
          />
          <button onClick={handleSearch} disabled={isSearching}>
            {isSearching ? "..." : "🔍"}
          </button>
        </div>
      </div>

      <Breadcrumb path={currentPath} onNavigate={handleNavigate} />

      {error && <div className="drive-error">{error}</div>}

      <div className="drive-content">
        <div className="drive-list">
          {loading ? (
            <div className="drive-loading">Loading...</div>
          ) : searchResults.length > 0 ? (
            <>
              <div className="drive-search-header">
                Search results for "{searchQuery}"
                <button onClick={() => { setSearchResults([]); setSearchQuery(""); }}>
                  Clear
                </button>
              </div>
              {searchResults.map((node) => (
                <div
                  key={node.path}
                  className={`drive-node drive-node-${node.node_type}`}
                  onClick={() => handleNodeClick(node)}
                >
                  <NodeIcon node={node} />
                  <span className="drive-node-name">{node.name}</span>
                  <span className="drive-node-path">{node.path}</span>
                </div>
              ))}
            </>
          ) : nodes.length === 0 ? (
            <div className="drive-empty">Empty directory</div>
          ) : (
            nodes.map((node) => (
              <div
                key={node.path}
                className={`drive-node drive-node-${node.node_type}`}
                onClick={() => handleNodeClick(node)}
              >
                <NodeIcon node={node} />
                <span className="drive-node-name">{node.name}</span>
                {node.badge && <Badge label={node.badge} />}
                {node.modified && (
                  <span className="drive-node-modified">{node.modified}</span>
                )}
              </div>
            ))
          )}
        </div>

        {selectedFile && (
          <div className="drive-preview">
            <div className="drive-preview-header">
              <span className="drive-preview-path">{selectedFile.path}</span>
              <span className="drive-preview-mime">{selectedFile.mime}</span>
              {selectedFile.editable && <span className="drive-preview-editable">✏️ editable</span>}
              <button onClick={() => setSelectedFile(null)}>✕</button>
            </div>
            <FileContent response={selectedFile} />
          </div>
        )}
      </div>
    </div>
  );
}
