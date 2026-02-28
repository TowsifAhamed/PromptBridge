import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';

let lastPrompt = '';

export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.commands.registerCommand('vibecode.openPanel', () => openPanel()),
    vscode.commands.registerCommand('vibecode.consumeLastPrompt', async () => consumeLastPrompt()),
    vscode.commands.registerCommand('vibecode.copyPrompt', async () => {
      await vscode.env.clipboard.writeText(lastPrompt);
      vscode.window.showInformationMessage('Prompt copied to clipboard');
    }),
  );

  watchPromptFile();
}

async function consumeLastPrompt() {
  const root = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  if (!root) {
    vscode.window.showWarningMessage('Open a workspace folder first.');
    return;
  }
  const promptPath = path.join(root, '.vibecode', 'last_prompt.json');
  if (!fs.existsSync(promptPath)) {
    vscode.window.showWarningMessage('.vibecode/last_prompt.json not found');
    return;
  }
  const data = JSON.parse(fs.readFileSync(promptPath, 'utf8'));
  lastPrompt = data.prompt || '';
  openPanel(data.tool || 'clipboard', lastPrompt);
}

function openPanel(tool = 'clipboard', prompt = lastPrompt) {
  const panel = vscode.window.createWebviewPanel('vibecode', 'PromptBridge', vscode.ViewColumn.One, {});
  panel.webview.html = `
    <h2>PromptBridge</h2>
    <p>Tool: ${tool}</p>
    <pre>${escapeHtml(prompt)}</pre>
    <p>Commands:</p>
    <ul>
      <li>Run <code>VibeCode: Copy Prompt</code></li>
      <li>Open Continue/Copilot chat manually</li>
    </ul>
  `;
}

function watchPromptFile() {
  const watcher = vscode.workspace.createFileSystemWatcher('**/.vibecode/last_prompt.json');
  watcher.onDidChange(() => consumeLastPrompt());
  watcher.onDidCreate(() => consumeLastPrompt());
}

function escapeHtml(value: string): string {
  return value
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;');
}

export function deactivate() {}
