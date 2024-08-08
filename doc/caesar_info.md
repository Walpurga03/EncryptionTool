# Caesar-Chiffre

Die Caesar-Chiffre ist eine der einfachsten und bekanntesten Verschlüsselungstechniken. Sie wurde nach Julius Caesar benannt, der sie angeblich in seiner privaten Korrespondenz verwendet hat. Die Caesar-Chiffre ist eine Substitutionschiffre, bei der jeder Buchstabe im Klartext durch einen Buchstaben ersetzt wird, der im Alphabet um eine feste Anzahl von Positionen verschoben ist.

## Funktionsweise

Die Caesar-Chiffre funktioniert, indem jeder Buchstabe des Klartexts um eine feste Anzahl von Positionen im Alphabet verschoben wird. Zum Beispiel, wenn der Verschiebungswert 3 ist, wird 'A' zu 'D', 'B' zu 'E', und so weiter. Am Ende des Alphabets wird wieder von vorne begonnen, d.h. 'X' wird zu 'A', 'Y' zu 'B', und 'Z' zu 'C'.

### Verschlüsselung

Um einen Text zu verschlüsseln, wird jeder Buchstabe des Klartexts um die festgelegte Anzahl von Positionen verschoben. Nicht-alphabetische Zeichen bleiben unverändert.

Beispiel:
- Klartext: `HELLO`
- Verschiebung: `3`
- Verschlüsselter Text: `KHOOR`

### Entschlüsselung

Um einen verschlüsselten Text zu entschlüsseln, wird jeder Buchstabe des verschlüsselten Texts um die festgelegte Anzahl von Positionen in die entgegengesetzte Richtung verschoben.

Beispiel:
- Verschlüsselter Text: `KHOOR`
- Verschiebung: `3`
- Klartext: `HELLO`

## Implementierung in Rust

Die Implementierung der Caesar-Chiffre in Rust erfolgt durch die Funktion `caesar_cipher`, die den Text und den Verschiebungswert als Eingabeparameter erhält und den verschlüsselten oder entschlüsselten Text zurückgibt.