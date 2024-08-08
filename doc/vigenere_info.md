# Vigenere-Chiffre

Die Vigenere-Chiffre ist eine polyalphabetische Substitutionschiffre, die im 16. Jahrhundert von Blaise de Vigenère entwickelt wurde. Sie verwendet ein Schlüsselwort, um den Klartext zu verschlüsseln, indem jeder Buchstabe des Klartexts durch einen Buchstaben ersetzt wird, der um eine Anzahl von Positionen verschoben ist, die durch das Schlüsselwort bestimmt wird.

## Funktionsweise

Die Vigenere-Chiffre verwendet ein Schlüsselwort, das wiederholt wird, um die Länge des Klartexts zu erreichen. Jeder Buchstabe des Klartexts wird dann um die Anzahl von Positionen verschoben, die durch den entsprechenden Buchstaben des Schlüsselworts bestimmt wird.

### Verschlüsselung

Um einen Text zu verschlüsseln, wird jeder Buchstabe des Klartexts um die Anzahl von Positionen verschoben, die durch den entsprechenden Buchstaben des Schlüsselworts bestimmt wird. Nicht-alphabetische Zeichen bleiben unverändert.

Beispiel:
- Klartext: `HELLO`
- Schlüsselwort: `KEY`
- Verschlüsselter Text: `RIJVS`

#### Schritt-für-Schritt Beispiel

1. Schreibe den Klartext auf: `HELLO`
2. Wiederhole das Schlüsselwort, um die Länge des Klartexts zu erreichen: `KEYKE`
3. Verschiebe jeden Buchstaben des Klartexts um die Position des entsprechenden Buchstabens im Schlüsselwort:
   - H (7) + K (10) = R (17)
   - E (4) + E (4) = I (8)
   - L (11) + Y (24) = J (9)
   - L (11) + K (10) = V (21)
   - O (14) + E (4) = S (18)

### Entschlüsselung

Um einen verschlüsselten Text zu entschlüsseln, wird jeder Buchstabe des verschlüsselten Texts um die Anzahl von Positionen in die entgegengesetzte Richtung verschoben, die durch den entsprechenden Buchstaben des Schlüsselworts bestimmt wird.

Beispiel:
- Verschlüsselter Text: `RIJVS`
- Schlüsselwort: `KEY`
- Klartext: `HELLO`

#### Schritt-für-Schritt Beispiel

1. Schreibe den verschlüsselten Text auf: `RIJVS`
2. Wiederhole das Schlüsselwort, um die Länge des verschlüsselten Texts zu erreichen: `KEYKE`
3. Verschiebe jeden Buchstaben des verschlüsselten Texts um die Position des entsprechenden Buchstabens im Schlüsselwort in die entgegengesetzte Richtung:
   - R (17) - K (10) = H (7)
   - I (8) - E (4) = E (4)
   - J (9) - Y (24) = L (11)
   - V (21) - K (10) = L (11)
   - S (18) - E (4) = O (14)