# Defining variables
In Rust poți declara variabilele într-o varietate de moduri, iar exemplele pe care le-ai dat sunt diferite moduri de a specifica tipul unei variabile sau de a-l lăsa pe Rust să-l deducă automat. Hai să le analizăm pe fiecare în parte:

### 1. **Declarația implicită a tipului (Rust deduce tipul)**:
   ```rust
   let a = 10;
   ```
   - **Rust deduce tipul** variabilei `a` pe baza valorii atribuite (în acest caz, `10`), care este de tipul `i32` deoarece `10` este un număr întreg și `i32` este tipul implicit pentru valori întregi.
   - Aici **nu se specifică tipul** explicit.

### 2. **Specificația explicită a tipului (adică `i32`)**:
   ```rust
   let b: i32 = 20;
   ```
   - Aici, tipul `i32` este specificat explicit pentru variabila `b`.
   - Este o declarație în care spui că `b` va fi întotdeauna un `i32`, iar Rust nu va încerca să deducă alt tip.

### 3. **Adăugarea unui sufix de tip (`i32`)**:
   ```rust
   let c = 30i32;
   ```
   - Aici, `30i32` adaugă explicit tipul `i32` la valoarea literală `30`. Este un mod de a specifica tipul pentru valoarea literală.
   - Rust va interpreta că `c` este de tip `i32` datorită sufixului `i32` adăugat la valoare.

### 4. **Folosirea unui separator de grupuri de cifre**:
   ```rust
   let d = 30_i32;
   ```
   - Acesta este un exemplu de utilizare a separatorului de grupuri de cifre `_` în valoarea literală `30`. `30_i32` este echivalent cu `30` de tip `i32`.
   - Separatorul `_` nu afectează valoarea numerică, dar este util pentru a face numere mari mai ușor de citit (de exemplu, `1_000_000` în loc de `1000000`).

### Sumar al diferitelor moduri de a declara variabile:
1. **Declarație implicită**: Rust deduce tipul pe baza valorii.
2. **Specificație explicită a tipului**: Tipul este specificat manual, folosind sintaxa `: tip`.
3. **Sufixul tipului pentru literal**: Folosind sufixe pentru a specifica tipul direct pe valoare (`30i32`).
4. **Separatorul `_` în literal**: Separă grupurile de cifre pentru a face numerele mai ușor de citit (`30_i32`).

### Concluzie:
Rust îți oferă flexibilitatea de a declara variabile folosind diverse stiluri, în funcție de preferințele tale și de cerințele specifice ale codului.
