# Architettura evolutiva di Sanpei

Stato: specifica per la prima implementazione; nessun componente applicativo ancora realizzato.

## 1. Moduli e dipendenze

| Modulo | Responsabilità | Dipendenze consentite |
| --- | --- | --- |
| Dominio | Uscite, intervalli, posizioni, misure, identificativi, vincoli | Librerie di base, nessuna infrastruttura |
| Contenuti | Schemi, cataloghi, evidenze, parametri, validazione pacchetti | Dominio |
| Motore | Valutazione deterministica, compatibilità, spiegazioni e differenze | Dominio e contratti dei contenuti |
| Servizi applicativi | Casi d'uso, transazioni, gestione revisioni e snapshot | Dominio, contenuti, motore e interfacce infrastrutturali |
| Persistenza | SQLx, SQLite, migrazioni, allegati e backup | Implementa le interfacce dei servizi |
| Provider | Meteo, mare e astronomia con dati normalizzati | Implementa le interfacce dei servizi |
| Desktop | Stato applicativo, comandi Tauri, permessi e dialoghi nativi | Servizi applicativi |
| UI | Svelte, MapLibre, visualizzazione e compilazione | Contratti applicativi TypeScript generati da Rust |

Usare moduli Rust interni separati; estrarre il nucleo in una libreria senza dipendenze desktop. Non introdurre microservizi, plugin binari o un framework generico di estensioni nella v1. La composizione dei provider avviene in un solo punto di avvio.

## 2. Contenuti e parametri aggiornabili

Ogni pacchetto contiene manifest, fonti, affermazioni, copertura GeoJSON, cataloghi, definizioni dei parametri, valori iniziali e regole dichiarative. Gli identificativi sono stabili e qualificati dal dominio; le etichette visibili sono modificabili senza cambiare identità.

Una definizione di parametro dichiara: identificativo, tipo (numero, booleano, categoria o intervallo), unità canonica quando applicabile, ambito, descrizione, vincoli strutturali e semantici, provenienza e revisione. Valori suggeriti e limiti di validazione sono concetti distinti. Un valore può essere sconosciuto: non sostituirlo automaticamente con zero o con un suggerimento.

Ogni affermazione scientifica o operativa conserva fonti, localizzazione nel documento, etichetta di evidenza, contesto studiato, limiti di trasferibilità, revisione e ambito geografico/temporale. Le quattro etichette iniziali del vademecum non diventano percentuali né pesi numerici arbitrari.

Separare i seguenti insiemi:

- Configurazioni operative: montature, intervalli pratici, varianti e protocolli di controllo.
- Evidenze: descrizioni documentate e inferenze dichiarate.
- Preferenze: obiettivi e vincoli scelti nell'app, distinti dalle scelte editoriali della ricerca.
- Informazioni variabili nel tempo: avvisi, normative, accessi e monitoraggi, con territorio e date di validità noti; validità sconosciuta resa visibile.

La v1 carica un pacchetto composito attivo per regione: il bundle di una regione risolve esplicitamente i propri cataloghi e parametri. L'importazione non combina implicitamente valori da ricerche concorrenti. Una futura sintesi di fonti produce una nuova revisione editoriale con conflitti risolti e motivazioni registrate.

## 3. Regole e capacità del motore

Le condizioni usano operatori tipizzati predefiniti: confronto, appartenenza, intervallo, presenza/assenza e combinazioni logiche. Le azioni iniziali selezionano o escludono candidati, suggeriscono varianti e producono motivazioni o verifiche richieste. Nessun `eval`, SQL, accesso alla rete o script dai pacchetti.

Ogni regola dichiara parametri utilizzati, ambito, evidenze e capacità richieste. Parametri assenti producono un esito esplicito di conoscenza insufficiente. Vincoli dell'inventario non dimostrati non autorizzano una compatibilità positiva. Le esclusioni editoriali sono distinguibili da divieti normativi verificati e da scelte dell'utente.

Applicare prima i vincoli di fattibilità, poi le preferenze dichiarate e infine le priorità operative documentate. Mostrare ogni motivo. Usare identificativi stabili per risolvere parità in modo deterministico; nessun punteggio viene presentato come probabilità di cattura.

Il manifest dichiara versione dello schema, revisione del contenuto e capacità minime richieste. Un pacchetto con operatori o significati non supportati non viene attivato; resta disponibile il precedente. Un parametro aggiunto può essere raccolto o visualizzato attraverso tipi già supportati, ma modifica le decisioni solo se regole compatibili lo utilizzano.

## 4. Persistenza e compatibilità

Conservare in tabelle normalizzate i concetti strutturali: uscite, canne, intervalli, catture, allegati, riferimenti alle posizioni e ai pacchetti. Per misure aggiuntive utilizzare un contenitore JSON validato, associato alla versione delle definizioni. Non trasformare tutto il database in coppie chiave-valore e non aggiungere una colonna per ogni parametro scientifico.

Versionare separatamente:

1. Schema e revisione del pacchetto scientifico.
2. Schema del database, con migrazioni ordinate e transazionali.
3. Versione del motore che ha prodotto una valutazione.
4. Schema degli snapshot di piano e dei formati di esportazione/backup.

Una fotografia di piano conserva input, provenienza dei valori, parametri effettivi, scelte bloccate, condizioni note in quel momento, risultati, motivazioni e versioni utilizzate. Conservare i pacchetti referenziati. La visualizzazione storica usa il risultato salvato: non dipende dalla possibilità di eseguire il vecchio motore.

La rivalutazione crea un nuovo piano collegato al precedente e un confronto esplicito; non sostituisce lo snapshot originario. Il backup include anche contenuti e definizioni necessarie a interpretare gli storici. Formati futuri incompatibili vengono respinti prima di modificare i dati; nessun campo sconosciuto deve essere scartato silenziosamente.

## 5. Cambiamenti futuri: comportamento previsto

| Cambiamento | Intervento |
| --- | --- |
| Nuova lunghezza suggerita, esca, variante o priorità documentata | Nuova revisione del pacchetto; nessuna modifica al motore se le capacità bastano |
| Nuova regione | Bundle regionale, geometrie verificate e revisione editoriale; nessuno sblocco automatico |
| Nuovo parametro di tipo già supportato | Definizione, presentazione generica e regole compatibili; assenza esplicita nei vecchi dati |
| Nuova operazione decisionale o nuova interazione complessa | Codice del modulo responsabile, capacità del motore e test; aggiornamento dell'app |
| Nuova struttura persistente | Migrazione SQLite e lettori compatibili; prova su database precedente |
| Nuovo provider | Adattatore sostituibile; non cambia il contratto ambientale senza una revisione esplicita |

## 6. Test architetturali di accettazione

- Modificare un parametro in un nuovo pacchetto cambia una nuova valutazione e lascia immutato il piano precedente.
- Aggiungere un parametro supportato non richiede una colonna SQLite; i vecchi intervalli lo mostrano come non registrato.
- Unità incompatibili, valori fuori dominio e riferimenti inesistenti bloccano l'attivazione senza importazione parziale.
- Una capacità sconosciuta conserva operativo il vecchio pacchetto; nessuna regola viene ignorata silenziosamente.
- Stessi input, pacchetto e motore producono gli stessi risultati e le stesse spiegazioni.
- Un aggiornamento dell'app e un backup/ripristino preservano piani, pacchetti storici e campi aggiuntivi supportati.
- Test del nucleo eseguibili senza WebView, rete o database; test d'integrazione separati per gli adattatori.
