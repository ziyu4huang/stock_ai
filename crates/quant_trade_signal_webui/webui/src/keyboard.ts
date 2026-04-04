// Keyboard shortcut handler

export function init(sendCommand: (action: string, extra?: Record<string, unknown>) => void): void {
  document.addEventListener('keydown', (e: KeyboardEvent) => {
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

    if (e.key >= '1' && e.key <= '9') {
      sendCommand('switch_tab', { index: parseInt(e.key) - 1 });
    } else if (e.key === 'ArrowRight' || e.key === 'Tab') {
      e.preventDefault();
      sendCommand('next_tab');
    } else if (e.key === 'ArrowLeft') {
      e.preventDefault();
      sendCommand('prev_tab');
    } else if (e.key === 'p' || e.key === 'P') {
      sendCommand('toggle_pause');
    } else if (e.key === 'a' || e.key === 'A') {
      sendCommand('toggle_auto');
    } else if (e.key === 's' || e.key === 'S') {
      sendCommand('toggle_sound');
    } else if (e.key === 'v' || e.key === 'V') {
      sendCommand('toggle_voice');
    } else if (e.key === ' ') {
      e.preventDefault();
      sendCommand('clear_alerts');
    }
  });
}
