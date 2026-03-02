const DEFAULT_HTTP_URL = "http://127.0.0.1:17777/run";

export async function runVibeCode({
  tool = "clipboard",
  prompt,
  repo,
  files,
  metadata,
  mode = "auto",
  http = {},
  onStatus = () => {},
  onError = () => {},
}) {
  const request = { tool, prompt, repo, files, metadata };
  if (!prompt) {
    const error = new Error("prompt is required");
    onError(error);
    throw error;
  }

  const protocolUrl = toProtocolUrl(request);

  if (mode === "protocol" || mode === "auto") {
    onStatus({ phase: "protocol", message: "Opening PromptBridge..." });
    try {
      window.location.href = protocolUrl;
      await wait(800);
    } catch (error) {
      onError(error);
    }
  }

  if (mode === "http" || mode === "auto") {
    onStatus({ phase: "http", message: "Trying localhost fallback..." });
    try {
      const response = await fetch(http.url || DEFAULT_HTTP_URL, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          ...(http.token ? { Authorization: `Bearer ${http.token}` } : {}),
        },
        body: JSON.stringify(request),
      });

      if (!response.ok) {
        throw new Error(`HTTP fallback failed: ${response.status}`);
      }

      return await response.json();
    } catch (error) {
      onError(error);
      if (mode === "http") throw error;
    }
  }

  const installError = {
    code: "PROMPTBRIDGE_NOT_AVAILABLE",
    message: "Install PromptBridge and start the desktop app.",
    installUrl: "https://github.com/your-org/promptbridge",
  };
  onError(installError);
  throw installError;
}

export function createRunButton(buttonEl, opts) {
  buttonEl.addEventListener("click", async () => {
    buttonEl.disabled = true;
    try {
      await runVibeCode(opts());
    } finally {
      buttonEl.disabled = false;
    }
  });
}

function toProtocolUrl({ tool, prompt, repo, files }) {
  const params = new URLSearchParams({ tool, prompt });
  if (repo) params.set("repo", repo);
  if (Array.isArray(files) && files.length) params.set("files", files.join(","));
  return `vibecode://run?${params.toString()}`;
}

function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
