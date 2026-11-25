# Pacchetti di ricerca

Un pacchetto è un file JSON locale preparato e revisionato fuori dall'app. Per generare un modello preliminare dalla ricerca iniziale:

```sh
python3 scripts/create_package.py > /tmp/sanpei-pacchetto.json
```

Cambiare identificativo e versione, aggiornare le fonti e rivedere ciascuna scheda e regola. Solo dopo la revisione impostare `review_status` a `reviewed`. In Dati e impostazioni scegliere **Importa e attiva pacchetto JSON**: Sanpei controlla la struttura e chiede conferma prima dell'attivazione. La validazione non verifica la veridicità scientifica, l'attualità normativa o i diritti delle fonti.

Lo schema 1 supporta attualmente soltanto il Lazio, schede testuali di settori e montature, regole per tecnica, condizione qualitativa, peso indicativo del piombo e montatura, e parametri aggiuntivi tipizzati. Non supporta geometrie, operatori arbitrari o script. Campi e operatori non supportati sono respinti: un nuovo tipo di regola richiede una modifica dell'app e test. Non usare nomi o versioni già importati per contenuti diversi.

Ogni nuova versione resta distinta; si possono riattivare quelle precedentemente importate o tornare ai contenuti iniziali. I piani conservano scenario, risultati e copia dei cataloghi e delle regole usate. Il pacchetto iniziale è preliminare, senza posizioni di pesca verificate.

## Parametri aggiuntivi

`parameters` è una lista facoltativa. Ogni elemento ha `id`, `label`, `kind` (`number`, `boolean` o `category`) e `source`. I numerici possono avere `unit`, `minimum` e `maximum`; le categorie richiedono `options`, una lista di valori distinti. Esempio:

```json
{"id":"trasparenza","label":"Trasparenza osservata","kind":"category","source":"Fonte revisionata da indicare","options":["limpida","torbida"]}
```

Una regola può contenere `predicates`, una lista di condizioni come `{"parameter":"trasparenza","operator":"eq","value":"limpida"}`. Per numeri sono disponibili `eq`, `neq`, `lt`, `lte`, `gt`, `gte`; per categorie e booleani solo `eq` e `neq`. Le condizioni di una regola sono tutte necessarie. Se un valore non è stato inserito, la regola interessata non produce una proposta e l'app segnala il dato mancante. Non si definiscono valori predefiniti impliciti. I parametri seguono l'origine osservazione/simulazione selezionata nello scenario.

Gli identificativi delle montature e dei settori devono essere stabili fra versioni. Campi non riconosciuti, riferimenti mancanti, tipi errati e operatori non supportati bloccano l'importazione. Versioni dell'app precedenti all'introduzione dei parametri potrebbero non accettare pacchetti che li includono.

## Schemi delle montature

Una scheda può includere `diagram`, facoltativo. `kind: "bottom"` richiede `main_cm` e una o due voci `branches` con `anchor_cm` dall'attacco del piombo e `length_cm`. `kind: "sliding"` richiede `terminal_cm`. `kind: "float"` richiede `float_g`, `terminal_cm` e `weights`, con `height_cm` dalla giunzione e `mass_g`; ogni peso può indicare `kind: "pellet"` o `"torpille"`. Tutte le misure devono essere positive, salvo l'altezza dello snodo che può essere zero; gli snodi devono stare sul trave. L'app disegna schemi non in scala dai valori del pacchetto e rifiuta forme o campi non supportati. Le misure e la coerenza con le descrizioni restano oggetto della revisione umana.
