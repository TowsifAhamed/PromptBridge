export function runVibeCode({ tool = "codex", prompt = "" }) {
  const encodedPrompt = encodeURIComponent(prompt);
  const url = `vibecode://run?tool=${tool}&prompt=${encodedPrompt}`;
  window.location.href = url;
}
