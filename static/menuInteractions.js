export function setupMenuInteractions(menuId, matrixTextId) {
  const menu = document.getElementById(menuId);
  const matrixText = document.getElementById(matrixTextId);

  window.selectMethod = function(method) {
      // Verstecke das Menü und zeige die ausgewählte Methode an
      menu.style.display = 'none';
      matrixText.textContent = `Ausgewählte Methode: ${method}`;

      // Hier könntest du den nächsten Schritt basierend auf der ausgewählten Methode einleiten
      // z.B. eine Eingabeaufforderung anzeigen oder direkt die Verschlüsselung starten
  };
}