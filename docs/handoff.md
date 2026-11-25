# Passaggio di consegne

## Stato attuale

- Repository pubblica creata; consultare Git per commit e stato di sincronizzazione.
- Scheletro Tauri 2 / Svelte / TypeScript / Vite / SQLx / SQLite e lockfile presenti.
- Mappa MapLibre offline con confini regionali ISTAT/Openpolis attribuiti, scelta della regione anche via elenco e rispetto del movimento ridotto. Non è ancora una base PMTiles dettagliata e non contiene settori o postazioni verificati.
- Diario iniziale: creazione/elenco uscite, canne, intervalli, catture, esito incompleto/zero/catture. I vincoli SQL vietano intervalli sovrapposti sulla stessa canna. Si possono registrare osservazioni datate. Mancano modifica e cancellazione, pause, posizioni strutturate, allegati e calcoli dello sforzo.
- Sei schede sintetiche preliminari in `content/lazio-rigs.v1.json`. Non sono un pacchetto scientifico completo né validato, e non includono tutte le misure.
- Richiesta esplicita a Open-Meteo per meteo/mare con coordinate e cache locale. Mancano normalizzazione completa, astronomia, test del provider, gestione di tutte le finestre e attribuzioni puntuali dei modelli.
- Pianificatore spiegabile, importazione/versionamento contenuti ed esportazioni non implementati. Copia e ripristino SQLite sono disponibili; le finestre native non sono ancora state provate nella WebView. Non dichiarare nessuna fase T0–T5 completa.

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
