# Passaggio di consegne

## Stato attuale

- Repository pubblica creata; consultare Git per commit e stato di sincronizzazione.
- Scheletro Tauri 2 / Svelte / TypeScript / Vite / SQLx / SQLite e lockfile presenti.
- Mappa MapLibre offline con confini regionali ISTAT/Openpolis attribuiti, scelta della regione anche via elenco e rispetto del movimento ridotto. Non è ancora una base PMTiles dettagliata e non contiene settori o postazioni verificati.
- Diario iniziale: creazione/elenco uscite, canne, intervalli, catture, esito incompleto/zero/catture. I vincoli SQL vietano intervalli sovrapposti sulla stessa canna. Mancano modifica e cancellazione, pause, posizioni strutturate, allegati, osservazioni e calcoli dello sforzo.
- Sei schede sintetiche preliminari in `content/lazio-rigs.v1.json`. Non sono un pacchetto scientifico completo né validato, e non includono tutte le misure.
- Richiesta esplicita a Open-Meteo per meteo/mare con coordinate e cache locale. Mancano normalizzazione completa, astronomia, test del provider, gestione di tutte le finestre e attribuzioni puntuali dei modelli.
- Pianificatore spiegabile, importazione/versionamento contenuti, backup/ripristino ed esportazioni non implementati. Non dichiarare nessuna fase T0–T5 completa.

## Verifiche e limiti

- Build frontend, Rust check, test e privacy eseguiti nel corso dello sviluppo; ripetere dopo ogni modifica. Il test attuale verifica sovrapposizioni e simultaneità delle canne.
- La prima versione del processo Tauri è stata avviata. La mappa e la navigazione sono state verificate visivamente nel browser, non ancora nella WebView Tauri; il diario e la persistenza dopo riavvio non sono stati verificati nell’app desktop.
- `pnpm tauri build --bundles deb` ha creato il pacchetto Debian; installazione pulita e funzionamento del pacchetto non sono stati provati.
- La ricerca originale resta esterna al repository; fonti primarie, licenze e coordinate delle postazioni non sono state ancora riverificate. Anomalia Pomezia A002 sospesa.
- Nessun backup/ripristino operativo: non usare l'app per dati insostituibili senza copie esterne.
- I controlli privacy automatici richiedono sempre una revisione manuale; non inserire dati personali o PDF nel repository.

## Prossimo compito

Verificare nell'app desktop la mappa locale, la creazione degli intervalli e la persistenza dopo riavvio usando dati sintetici. Completare il diario e la conservazione sicura prima della release; quindi implementare pacchetti scientifici validati e motore spiegabile secondo `implementation-plan.md` e `architecture.md`.

## Tentativo di backup

Una prima implementazione `VACUUM INTO` non ha superato il test di lettura e non è stata pubblicata. Una prova separata con database su disco e destinazione inesistente ha prodotto una copia leggibile; il caso della destinazione preesistente rimane da verificare. Implementare pubblicazione atomica senza sovrascrittura e testare ripristino prima di esporre la funzione.
