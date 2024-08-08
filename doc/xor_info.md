# XOR-Verschlüsselung
Die XOR-Verschlüsselung ist eine einfache und häufig verwendete Methode in der Kryptographie. Der Name "XOR" steht für "Exclusive OR", eine logische Operation, die auf binäre Daten angewendet wird.

## Grundprinzip
Die XOR-Operation vergleicht zwei Bits. Wenn die Bits unterschiedlich sind, ist das Ergebnis `1`, andernfalls ist es `0`. Die Wahrheitstabelle für XOR sieht wie folgt aus:

| A | B | A XOR B |
|---|---|---------|
| 0 | 0 |    0    |
| 0 | 1 |    1    |
| 1 | 0 |    1    |
| 1 | 1 |    0    |

## Anwendung in der Verschlüsselung
Die XOR-Verschlüsselung funktioniert, indem jedes Bit des Klartextes mit einem Bit des Schlüssels XOR-verknüpft wird. Wenn der Schlüssel genauso lang wie der Klartext ist und nur einmal verwendet wird, spricht man von einem One-Time Pad, das theoretisch unknackbar ist. 

### Verschlüsselung und Entschlüsselung
Die Verschlüsselung und Entschlüsselung mit XOR sind identisch. Da die XOR-Operation involutiv ist (d.h. `A XOR B XOR B = A`), kann derselbe Algorithmus sowohl zur Verschlüsselung als auch zur Entschlüsselung verwendet werden.

### Beispiel
Angenommen, wir haben folgenden Klartext und Schlüssel:

- Klartext: `1010 1100`
- Schlüssel: `0110 1001`

Die XOR-Verschlüsselung erfolgt bitweise:

```
Klartext: 10101100
Schlüssel: 01101001
----------------
Ergebnis : 11000101
```

Der resultierende verschlüsselte Text ist `1100 0101`.

Um zu entschlüsseln, wenden wir den gleichen Schlüssel auf den verschlüsselten Text an:

```
Verschlüsselt: 11000101
Schlüssel    : 01101001
----------------
Ergebnis     : 10101100
```

Der entschlüsselte Text ist `1010 1100`, was dem ursprünglichen Klartext entspricht.

### Vorteile
- **Einfachheit**: XOR ist einfach zu implementieren und schnell zu berechnen.
- **Symmetrie**: Der gleiche Algorithmus wird für Verschlüsselung und Entschlüsselung verwendet.

### Nachteile
- **Schlüssellänge und Wiederholung**: Wenn der Schlüssel kürzer als der Klartext ist und wiederholt verwendet wird, kann die Verschlüsselung leicht gebrochen werden.
- **Schlüsselsicherheit**: Ein sicherer Schlüssel muss genauso lang sein wie der Klartext und darf nur einmal verwendet werden (One-Time Pad), um sicher zu sein.