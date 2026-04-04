// DOM helper utilities

export function el(id: string): HTMLElement {
  return document.getElementById(id)!;
}

/** Set textContent only if changed — avoids unnecessary DOM mutations */
export function setText(id: string, text: string): void {
  const e = el(id);
  if (e.textContent !== text) e.textContent = text;
}

/** Set innerHTML only if changed */
export function setHtml(id: string, html: string): void {
  const e = el(id);
  if (e.innerHTML !== html) e.innerHTML = html;
}

/** Escape HTML special characters */
export function esc(s: string): string {
  return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
}
