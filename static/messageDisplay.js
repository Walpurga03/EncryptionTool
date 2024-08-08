export function startMessageDisplay(cursorId, matrixTextId, messages, callback) {
  let blinkCount = 0;
  const cursor = document.getElementById(cursorId);
  const matrixText = document.getElementById(matrixTextId);
  let currentMessageIndex = 0;

  function blinkCursor() {
      cursor.style.opacity = cursor.style.opacity === '0' ? '1' : '0';
      blinkCount++;
      if (blinkCount < 10) { // 10 Wechsel = 5 Mal blinken
          setTimeout(blinkCursor, 500);
      } else {
          cursor.style.display = 'none';
          showMessage();
      }
  }

  function showMessage() {
      if (currentMessageIndex < messages.length) {
          const message = messages[currentMessageIndex];
          let index = 0;
          matrixText.textContent = ''; // Lösche den vorherigen Text

          function typeNextCharacter() {
              if (index < message.length) {
                  matrixText.textContent += message[index++];
                  setTimeout(typeNextCharacter, 150);
              } else {
                  currentMessageIndex++;
                  setTimeout(() => {
                      cursor.style.display = 'inline-block';
                      blinkCursor(); // Blinke den Cursor erneut vor der nächsten Nachricht
                  }, 1000); // Eine Sekunde warten, bevor der nächste Text angezeigt wird
              }
          }

          typeNextCharacter();
      } else {
          // Alle Nachrichten angezeigt, jetzt das eigentliche Programm starten
          callback();
      }
  }

  setTimeout(blinkCursor, 500); // Startet den Blinkeffekt
}