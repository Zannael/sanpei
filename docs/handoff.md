# Passaggio di consegne

## Stato attuale

- Repository pubblica creata; consultare Git per commit e stato di sincronizzazione.
- Scheletro Tauri 2 / Svelte / TypeScript / Vite / SQLx / SQLite e lockfile presenti.
- Mappa MapLibre offline con confini regionali ISTAT/Openpolis attribuiti, scelta della regione anche via elenco e rispetto del movimento ridotto. Include schede sintetiche dei settori della ricerca, senza geometrie precise. Non è ancora una base PMTiles dettagliata e non contiene postazioni verificate.
- Diario iniziale: creazione/elenco uscite, canne, intervalli, catture, esito incompleto/zero/catture. I vincoli SQL vietano intervalli sovrapposti sulla stessa canna. Si possono registrare osservazioni datate, modificare o eliminare un’uscita ed eliminare singoli elementi con conferma. Gli intervalli distinguono pause, minuti con esca inefficace e tempo effettivo, che resta sconosciuto se mancano dati. Postazione e zona di lancio hanno coordinate personali, origine e accuratezza; la mappa regionale non verifica i dettagli. Mancano modifica dei singoli elementi, allegati e aggregati dello sforzo.
- Sei schede sintetiche preliminari in `content/lazio-rigs.v1.json`. Non sono un pacchetto scientifico completo né validato, e non includono tutte le misure.
- Richiesta esplicita a Open-Meteo per meteo/mare con coordinate e cache locale. Mancano normalizzazione completa, astronomia, test del provider, gestione di tutte le finestre e attribuzioni puntuali dei modelli.
- Il pianificatore iniziale usa poche regole operative, condizioni inserite manualmente, tecnica, settore e limite della canna; salva scenario, risultati e cataloghi come snapshot. Un piano può creare un’uscita, essere collegato al diario e registrare se è stato seguito. Si può allegare e consultare la risposta meteo/mare disponibile nello snapshot, ma non influenza le regole. È possibile adattare un piano e confrontare input, versioni e montature aggiunte/rimosse; non usa ancora la specie o finestre confrontate e non interpreta automaticamente le osservazioni del diario. I pacchetti JSON Lazio dello schema 1 possono essere validati, importati, attivati e riattivati; la validazione strutturale non verifica la correttezza scientifica. Sono disponibili esportazioni JSON, CSV per uscite/intervalli/catture e GeoJSON delle posizioni; gli allegati non sono ancora gestiti. Copia e ripristino SQLite sono disponibili; le finestre native non sono ancora state provate nella WebView. Non dichiarare nessuna fase T0–T5 completa.

## Verifiche e limiti

- Build frontend, Rust check, test e privacy eseguiti nel corso dello sviluppo; ripetere dopo ogni modifica. Il test attuale verifica sovrapposizioni e simultaneità delle canne.
- La prima versione del processo Tauri è stata avviata. La mappa e la navigazione sono state verificate visivamente nel browser, non ancora nella WebView Tauri; il diario e la persistenza dopo riavvio non sono stati verificati nell’app desktop.
- `pnpm tauri build --bundles deb` ha creato il pacchetto Debian; installazione pulita e funzionamento del pacchetto non sono stati provati.
- La ricerca originale resta esterna al repository; fonti primarie, licenze e coordinate delle postazioni non sono state ancora riverificate. Anomalia Pomezia A002 sospesa.
- Copia e ripristino SQLite sono verificati da test su disco; le finestre native e il flusso end-to-end restano da provare.
- I controlli privacy automatici richiedono sempre una revisione manuale; non inserire dati personali o PDF nel repository.

## Prossimo compito

Verificare nell'app desktop la mappa locale, la creazione degli intervalli e la persistenza dopo riavvio usando dati sintetici. Completare il diario e la conservazione sicura prima della release; quindi implementare pacchetti scientifici validati e motore spiegabile secondo `implementation-plan.md` e `architecture.md`.

## Copia del database

La copia usa `VACUUM INTO` in una directory temporanea privata, controlla l'integrità e pubblica il file tramite hard link senza sovrascrivere. Test su database su disco coprono lettura e destinazione esistente. File system senza supporto agli hard link restituiscono errore. Il ripristino è ancora da progettare e verificare; non è presente nell'interfaccia.

## Orari e piani

Gli orari inseriti sono interpretati in Europe/Rome e convertiti in UTC; orari ambigui o inesistenti al cambio dell’ora vengono rifiutati. Le regole iniziali sono incorporate; i pacchetti revisionati dello schema 1 possono sostituirle e restano versionati localmente. La cronologia conserva scenario, risultato e copie delle regole, montature e settori utilizzati.

## Ricerca aggiornabile

Il formato e i limiti sono descritti in `docs/research-packages.md`. I test verificano rigetto di campi sconosciuti e riferimenti invalidi, importazione senza sovrascrittura, cambio delle nuove valutazioni e conservazione dei vecchi piani. La procedura verifica la struttura, non le fonti, le licenze o le conclusioni.

## Esportazioni

Il JSON contiene tabelle del diario, piani, snapshot delle regole, pacchetti e cache; campi booleani SQLite sono espressi come 0/1. I CSV sono separati per uscite, intervalli e catture, con campi mancanti vuoti e protezione delle formule. Il GeoJSON contiene posizioni personali: la destinazione va scelta consapevolmente. Salvataggio su file nuovo, con pubblicazione atomica tramite hard link.
