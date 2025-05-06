## Slide 1/21: Titolo

Buongiorno a tutti. L'obiettivo di questa tesi è stato esplorare e sviluppare nuovi metodi per eseguire query sui cammini in grafi aciclici diretti pesati, con un focus particolare sull'efficienza in termini di spazio.

## Slide 2: The Subset Membership Problem

Immaginiamo uno scenario tipico: abbiamo un vasto universo di elementi $U$, per semplicità gli interi da 1 a $n$, e da questo universo abbiamo selezionato un particolare sottoinsieme $S$, che contiene $m$ di questi elementi.

_(Click per far apparire l'alertblock)_

Il nostro compito principale è duplice. Primo, data questa collezione, vogliamo poter rispondere molto rapidamente alla domanda: "Un certo elemento $x$ fa parte del nostro sottoinsieme $S$?". Questa è la classica **query di appartenenza**. Secondo, e altrettanto importante, specialmente quando $n$ ed $m$ sono grandi, vogliamo che la struttura dati che usiamo per memorizzare le informazioni su $S$ occupi la minor quantità di spazio possibile: cerchiamo una **rappresentazione compatta**.

_(Click, Pausa 1)_

Per capire cosa significhi "spazio minimo", ci viene in aiuto la Teoria dell'Informazione. La domanda che ci poniamo è: qual è il numero minimo assoluto di bit di cui abbiamo bisogno per distinguere il nostro specifico sottoinsieme $S$ da tutti gli altri possibili sottoinsiemi di $m$ elementi che avremmo potuto scegliere? Questo concetto di contenuto informativo minimo ci porta naturalmente all'Entropia di Shannon, $H(X)$, che vedete definita qui. Essa misura, per una sorgente generica $X$, l'incertezza media, o il contenuto informativo medio, associato a ciascun simbolo emesso.

---

## Slide 3: Information-Theoretic Limits for Subsets

Il Teorema della Codifica di Sorgente di Shannon, che qui richiamiamo, ci dice che l'entropia $H(X)$ costituisce un limite teorico inferiore invalicabile per qualsiasi rappresentazione _lossless_ dei dati provenienti da una sorgente $X$.

_(Click, Pausa 1)_

Tuttavia, nella pratica, raramente conosciamo la vera distribuzione di probabilità $P_X$ della "sorgente" che genera i nostri dati o i nostri sottoinsiemi. Più comunemente, abbiamo a disposizione una sequenza finita di dati $S$ che abbiamo osservato. Per questa specifica sequenza, possiamo calcolare l'Entropia Empirica di Ordine Zero, $\mathcal{H}_0(S)$. Questa, come vedete dalla formula, si basa sulle frequenze $n_s/n$ con cui i simboli $s$ appaiono effettivamente nella sequenza, fornendoci un benchmark pratico sulla sua comprimibilità.

_(Click, Pausa 2)_

Ora, colleghiamo questo al nostro problema del sottoinsieme $S$ di $m$ elementi scelti da $n$. Il numero di modi distinti per scegliere $m$ elementi da un insieme di $n$ è dato dal coefficiente binomiale $\binom{n}{m}$. Per specificare univocamente quale di queste $\binom{n}{m}$ scelte abbiamo fatto, la teoria dell'informazione ci dice che sono necessari almeno $\lceil \log_2 \binom{n}{m} \rceil$ bit. Questo perché ogni bit raddoppia il numero di possibilità che possiamo distinguere; quindi, per distinguere $N$ possibilità, servono $\log_2 N$ bit. Questa quantità, $\lceil \log_2 \binom{n}{m} \rceil$, è proprio il contenuto informativo intrinseco della scelta del nostro particolare sottoinsieme.

---

## Slide 4: Bitvectors: Querying the Implicit Representation

Un modo naturale e diretto per rappresentare il nostro sottoinsieme $S$ è attraverso un **bitvector** $B$ di lunghezza $n$: semplicemente, il bit $B[i]$ è 1 se l'elemento $i$ appartiene a $S$, e 0 altrimenti. Il punto cruciale qui è _come_ lo memorizziamo. Invece di un array esplicito di $n$ bit, stiamo essenzialmente **codificando la scelta delle $m$ posizioni che contengono gli '1'**. Questo approccio, come visto, ci permette di memorizzare l'informazione sul bitvector $B$ utilizzando uno spazio di circa $\lceil \log_2 \binom{n}{m} \rceil$ bit. Sottolineo: il bitvector $B$ _non è memorizzato come un array esplicito di $n$ bit_.

_(Click, Pausa 1. Appare l'immagine `rank_select_1.pdf`.)_

Questo che vedete è il bitvector _concettuale_. La sua rappresentazione in memoria è invece implicita e molto più compatta.

_(Click, Pausa 2. Appare la domanda "If B is not explicit...".)_

Se $B$ non è memorizzato esplicitamente, come facciamo ad accedere a un suo elemento $B[i]$? Per farlo, introduciamo due query fondamentali:

_(Click, Pausa 3. Appare la definizione di `rank`.)_

La prima è **`rank`$_b(B, i)$**. Questa operazione conta quante volte il bit $b$ (0 o 1) appare nel prefisso del bitvector (quello implicito) fino alla posizione $i$.

_(Click, Pausa 4. L'immagine cambia in `rank_select_2.pdf`.)_

Ad esempio, come illustrato, `rank`$_1(B,15)$ ci dice quanti '1' ci sono fino alla posizione 15.

_(Click, Pausa 5. Appare la definizione di `select`.)_

La seconda operazione è **`select`$_b(B, j)$**, che ci fornisce la posizione (l'indice) del $j$-esimo bit $b$ nel bitvector.

_(Click, Pausa 6. L'immagine cambia in `rank_select_3.pdf`.)_

Ad esempio, `select`$_1(B,7)$ ci indica la posizione del settimo '1', che qui è 12.

_(Click, Pausa 7. Appare il blocco "Access Queries".)_

Ora, come usiamo `rank` per la query di appartenenza? Per sapere se $B[i]$ è '1', confrontiamo il numero di '1' fino a $i$ con il numero di '1' fino a $i-1$. Se `rank`$_1(B,i)$ è maggiore di `rank`$_1(B,i-1)$, significa che esattamente in posizione $i$ deve esserci un '1' che prima non c'era. Altrimenti, $B[i]$ è '0'. Semplice ma potente!

---

## Slide 5: RRR Structure: The Bitvector Solution

Abbiamo quindi visto che, per interrogare un bitvector memorizzato in forma compatta e rispondere a query di appartenenza, le operazioni di `rank` e `select` sono il nostro pane quotidiano. Nelle Strutture Dati Succinte per Bitvector, l'obiettivo è chiaro: vogliamo supportare `rank` e `select` in tempo costante, $O(1)$, e fare ciò utilizzando uno spazio che sia il più vicino possibile al minimo teorico per il bitvector, cioè $\lceil \log_2 \binom{n}{m} \rceil$ bit.

_(Click, Pausa 1)_

Un risultato fondamentale, quasi un punto di arrivo per questo problema specifico, è la struttura RRR. Il teorema che la definisce afferma che un bitvector $B$ con $m$ bit impostati a '1' può essere rappresentato utilizzando esattamente $B(n,m)$ bit (che è il nostro $\lceil \log_2 \binom{n}{m} \rceil$) più alcuni termini additivi di ordine inferiore, $o(n)$ e $O(\log \log n)$, che per $n$ grandi diventano trascurabili. E, cosa cruciale, questa rappresentazione ultra-compatta supporta le query di `rank` e `select` in tempo costante.

_(Click, Pausa 2)_

Questo è un risultato di grande impatto. La struttura RRR dimostra che non c'è necessariamente un trade-off tra estrema compattezza e velocità di interrogazione per il problema dei sottoinsiemi. È possibile ottenere uno spazio di memorizzazione ottimale _e contemporaneamente_ la capacità di eseguire query efficienti. Questo risultato incarna perfettamente la filosofia delle Strutture Dati Succinte.

---

## Slide 6: Why Succinct Data Structures?

La struttura RRR, che abbiamo appena visto risolvere elegantemente il problema della rappresentazione compatta e interrogabile dei bitvector, è in realtà una soluzione specifica a un problema molto più generale che affligge chiunque lavori con grandi moli di dati.

_(Click, testo del blocco "Massive Data & Auxiliary Structures Overhead" appare)_

I dataset con cui abbiamo a che fare oggi, in campi che vanno dalla ricerca scientifica al web, fino all'intelligenza artificiale, sono di dimensioni spesso gigantesche. Per analizzarli in modo complesso e interattivo, idealmente vorremmo che risiedessero nella memoria RAM, per un accesso il più rapido possibile. Tuttavia, le strutture dati ausiliarie – pensiamo a indici, alberi di ricerca, e simili – che costruiamo sopra i dati per poterli interrogare efficientemente, finiscono spesso per occupare uno spazio addizionale che può superare, a volte anche di molto, la dimensione dei dati originali stessi. Il risultato è che far stare tutto in RAM diventa un serio ostacolo, un vero e proprio collo di bottiglia prestazionale.

_(Click, appaiono le due colonne del "Classic Trade-off")_

Questo ci pone di fronte a un classico dilemma, un trade-off. Da un lato, potremmo ricorrere a tecniche di compressione dati tradizionali per minimizzare lo spazio occupato. Queste tecniche sono efficaci nel ridurre le dimensioni, ma tipicamente rendono le query dirette sui dati lenti o impossibili, perché richiedono una fase di decompressione, che sia essa totale o parziale. Dall'altro lato, le strutture dati classiche, che non usano compressione spinta, offrono query veloci ma, come abbiamo appena detto, lo fanno al costo di un considerevole overhead di spazio.

_(Click, appare il blocco "The Succinct Goal")_

L'obiettivo delle Strutture Dati Succinte, e il filo conduttore di questa tesi, è proprio quello di tentare di superare questo dilemma, cercando di ottenere il meglio da entrambi i mondi. La domanda che ci poniamo è: possiamo raggiungere entrambi questi obiettivi apparentemente contrastanti? Vogliamo, cioè, uno spazio di memorizzazione per i nostri dati e le nostre strutture che sia il più vicino possibile al minimo teorico dettato dalla teoria dell'informazione – quindi massima compattezza – e, allo stesso tempo, vogliamo poter eseguire query efficienti direttamente su questi dati mantenuti in forma compatta, senza la necessità di decomprimerli.

## Slide 7/21: Beyond Bitvectors: General Alphabets (Wavelet Trees)

Estendiamo ora questi concetti a sequenze definite su alfabeti di dimensione $> 2$, come sequenze testuali o biologiche. Come possiamo supportare efficientemente rank e select su questi alfabeti più grandi?

Una soluzione strutturale elegante è il **Wavelet Tree**. L'idea cardine è quella di ridurre ricorsivamente il problema su un alfabeto grande a una serie di problemi su alfabeti più piccoli, fino a ricondursi a operazioni su bitvector. A ogni nodo dell'albero, l'alfabeto corrente viene partizionato in due metà, e si costruisce un bitvector che codifica, per ogni simbolo della sequenza associata al nodo, a quale parte della partizione appartiene. Le sottosequenze corrispondenti vengono poi passate ricorsivamente ai figli. Questo processo genera un albero binario bilanciato di altezza $O(\log \sigma)$. Le query sull'alfabeto originale vengono tradotte in una sequenza di query rank/select sui bitvector memorizzati nei nodi lungo un percorso dalla radice a una foglia (o viceversa). Se i bitvector interni sono implementati con strutture compresse come RRR, l'intero Wavelet Tree diventa **compresso rispetto all'entropia** $H_0$ della sequenza originale, e le query richiedono tipicamente un tempo $O(\log \sigma)$.

## Slide 8/21: Representing Sequence Variation: Degenerate Strings

Consideriamo un'ulteriore generalizzazione: le **stringhe degeneri**. In queste sequenze, ogni posizione $i$ non è associata a un singolo simbolo, ma a un _sottoinsieme_ $X_i$ dell'alfabeto $\Sigma$. Questo modello trova applicazione, ad esempio, nella rappresentazione di incertezze o polimorfismi in sequenze biologiche. Anche in questo contesto, possiamo definire operazioni analoghe a rank e select, come `subset-rank` (contare gli insiemi $X_k$ fino a $i$ che contengono un dato carattere $c$) e `subset-select` (trovare l'indice $k$ del $j$-esimo insieme $X_k$ che contiene $c$).

[*click*]

Per supportare efficientemente queste query, possiamo impiegare una tecnica di **riduzione**. La stringa degenere viene trasformata in due strutture ausiliarie: una lunga sequenza $S$ ottenuta concatenando tutti i caratteri da tutti gli insiemi $X_i$, e un bitvector $R$ che funge da delimitatore tra le rappresentazioni dei diversi $X_i$ all'interno di $S$. Combinando operazioni di rank/select su $S$ (implementata con un Wavelet Tree) e su $R$ (implementato con una struttura per bitvector), è possibile rispondere alle query `subset-rank` e `subset-select` sulla stringa degenere originale.

---

## _(Transizione)_

Dopo aver esplorato l'applicazione di strutture succinte a diverse forme di dati sequenziali, introduciamo ora un cambio di paradigma. Mostreremo come il problema appena discusso delle stringhe degeneri possa essere riformulato nel linguaggio dei grafi, conducendoci al nucleo di questa tesi.

## Slide 9/21: From Degenerate Strings to Weighted DAGs

Consideriamo nuovamente la stringa degenere $X$.

[*click*]

Proponiamo di modellare le query relative a un carattere specifico $c$ attraverso la costruzione di un **Grafo Aciclico Diretto (DAG) con pesi sui nodi**, denotato $G_c$. La costruzione prevede: un nodo sorgente $s$; un vertice per ogni carattere in $X_k$ alla posizione $k$;

[*click*]

pesi assegnati ai vertici (1 se $a=c$, 0 altrimenti, 0 per $s$);

[*click*]

archi che collegano $s$ al primo livello e ogni nodo del livello $k$ a tutti i nodi del livello $k+1$.

[*click*]

Questa costruzione stabilisce una corrispondenza biunivoca tra i cammini da $s$ a un vertice al livello $k$ e le possibili realizzazioni della stringa degenere fino alla posizione $k$.

Crucialmente, la **somma dei pesi $w_c$ lungo un cammino conta precisamente le occorrenze del carattere $c$** in quella realizzazione. Tale equivalenza motiva lo studio di strutture dati per interrogare efficientemente i pesi cumulativi dei cammini nei DAG.

## Slide 10/21: Path Weight Aggregation: The O-Set

Generalizziamo ora al contesto dei DAG nodi-pesati. Dato un cammino $P$ da $s$ a $v$, definiamo il suo peso $W(P)$ come somma dei pesi dei nodi in $P$ (esclusa $s$). Ci interessa caratterizzare l'insieme di tutti i **valori distinti** di $W(P)$ per i cammini terminanti in $v$.

[*click*]

A questo scopo, introduciamo l'**$\mathcal{O}$-set** di $v$, $\mathcal{O}_v$, definito formalmente come l'insieme dei pesi $W(P)$ per tutti i cammini $P$ da $s$ a $v$, eliminando i duplicati. La sua costruzione è ricorsiva: $\mathcal{O}_s=\{0\}$; per $v \neq s$, $\mathcal{O}_v$ è l'unione, su tutti i predecessori $u$ di $v$, degli insiemi $\{y + w(v) \mid y \in \mathcal{O}_u\}$, mantenuta ordinata e senza duplicati.

## Slide 11/21: O-Set Construction Example

Illustriamo brevemente la costruzione. Partendo da $\mathcal{O}_0=\{0\}$, si calcolano gli O-set dei nodi successivi in ordine topologico. Ad esempio, per calcolare $\mathcal{O}_6$, si considerano i suoi predecessori 1 e 3 (con $\mathcal{O}_1=\{1\}, \mathcal{O}_3=\{3\}$) e $w(6)=6$. L'unione dei risultati $\{1+6\}$ e $\{3+6\}$ dà $\mathcal{O}_6=\{7, 9\}$. Il processo si itera per l'intero grafo.

## Slide 12/21: The Rank Query on Weighted DAGs

Definiamo ora la **query Rank** $\mathrm{rank}_G(N)$, centrale per il nostro lavoro. A differenza dell'O-set, che si concentra sui pesi _finali_ dei cammini, la query Rank caratterizza l'insieme dei pesi cumulativi _durante_ l'attraversamento del nodo $N$. Per ogni peso finale $x \in \mathcal{O}_N$, si considera l'intervallo di pesi parziali $[\max(0, x - w(N) + 1), x]$ che potevano condurre a $x$ sommando $w(N)$.

[*click*]

Si calcola l'unione $S_N$ di tutti questi intervalli e se ne determina la **rappresentazione canonica** come unione minima di intervalli chiusi e disgiunti, $\mathcal{R}_N$. Questo $\mathcal{R}_N$ è il risultato della query Rank.

## Slide 13/21: The Challenge: Storing Path Information

Tuttavia, emerge una sfida computazionale: la cardinalità degli O-set può crescere esponenzialmente in certi DAG, rendendo la loro memorizzazione esplicita proibitiva.

[*click*]

Per superare questo limite, proponiamo una strategia basata su **Partizionamento** dei nodi e **Indirezione**. I nodi vengono divisi in due categorie. La prima è quella dei **Nodi Espliciti** ($V_E$). Nella nostra implementazione attuale, abbiamo scelto come nodi espliciti **soltanto i nodi sink** del grafo, ovvero quelli senza successori; per questi, memorizziamo direttamente il loro O-set completo.

_(Potremmo notare che, per ottimizzare ulteriormente lo spazio, sarebbe anche possibile introdurre un unico nodo "dummy" sink fittizio, con peso zero, a cui collegare tutti i sink originali. In questo modo, dovremmo memorizzare esplicitamente un solo O-set potenzialmente grande, quello del nodo dummy, invece di uno per ogni sink originale.)_

La seconda categoria è quella dei **Nodi Impliciti** ($V_I$), che comprendono tutti gli altri vertici del grafo.

[*click*]

Per questi nodi impliciti, evitiamo la memorizzazione esplicita dell'O-set e implementiamo un meccanismo per ricostruirlo al bisogno, come vedremo tra poco.

## Slide 14/21: Implicit Reconstruction: Successor & Offset

La ricostruzione per un nodo implicito $v$ si basa su due informazioni memorizzate: un puntatore a un **Successore Designato** $\sigma(v)$, scelto euristicamente per minimizzare la cardinalità $|\mathcal{O}_{\sigma(v)}|$,

e una **Sequenza di Offset** $\mathcal{J}_v$. Quest'ultima codifica la relazione $x_k = y_{j_k} - w(\sigma(v))$ tra l'$k$-esimo elemento $x_k$ di $\mathcal{O}_v$ e il $j_k$-esimo elemento $y_{j_k}$ di $\mathcal{O}_{\sigma(v)}$, memorizzando l'indice $j_k$.

## Slide 15/21: Example: Computing O7[0]

Illustriamo la ricostruzione del valore $\mathcal{O}_7[0]$. Partendo da 7 (implicito), si segue il successore designato $\sigma(7)=9$. Si consulta l'offset $\mathcal{J}_7[0]$ per ottenere un indice $j$ relativo a $\mathcal{O}_9$. Si accumula il peso $w(9)$. Si ripete il processo da 9 (implicito), seguendo $\sigma(9)=5$, usando $\mathcal{J}_9[j]$ per ottenere un indice $k$ per $\mathcal{O}_5$, e accumulando $w(5)$. Si prosegue da 5 (implicito) a $\sigma(5)=8$, usando $\mathcal{J}_5[k]$ per ottenere un indice $l$ per $\mathcal{O}_8$, e accumulando $w(8)$. Poiché 8 è Esplicito, si recupera il valore $\mathcal{O}_8[l]$. Il risultato finale è $\mathcal{O}_8[l]$ meno la somma dei pesi accumulati.

## Slide 16/21: Succinct Data Structure: Components

In sintesi, la nostra struttura dati per il DAG consiste, per ogni nodo $i$, nel suo peso $w(i)$, nel riferimento al successore $\sigma(i)$, e nei dati associati $\mathcal{D}[i]$ (contenenti $\mathcal{O}_i$ o $\mathcal{J}_i$). Questa organizzazione come "Struct of Arrays" facilita la compressione separata dei componenti.

## Slide 17/21: Compression Strategies

Per la compressione, utilizziamo tecniche appropriate per ciascun componente. Per i pesi ($\mathcal{W}$) e i successori ($\Sigma$), si possono impiegare codifiche intere a lunghezza variabile (VLC). Per i dati associati ($\mathcal{D}$), che consistono in sequenze monotone crescenti (O-set e offset), sono particolarmente indicate tecniche come Elias-Fano encoding o Run-Length Encoding (RLE), quest'ultima vantaggiosa in presenza di lunghe sequenze di interi consecutivi.

## Slide 18/21: Space Efficiency: Baseline Comparison

Per valutare l'efficienza spaziale della nostra struttura, introduciamo un termine di paragone: l'**Entropia di Ordine Zero del Grafo**, $\mathcal{H}_0(G)$. Essa rappresenta il limite inferiore teorico allo spazio richiesto per memorizzare l'intera struttura del grafo pesato (vertici, archi, pesi) in modo lossless. Si compone del costo per codificare i pesi,

[*click*]

$\mathcal{H}_W(G)$,

[*click*]

e del costo per codificare la topologia (l'insieme degli archi), $\mathcal{H}_E(G)$.

[*click*]

Qualsiasi rappresentazione lossless dell'intero grafo deve occupare almeno $\mathcal{H}_0(G)$ bit.

## Slide 19/21: Space Comparison: Succinct Structure vs. Baselines

Presentiamo ora un confronto spaziale basato su un DAG derivato dalla rete Bitcoin. L'entropia $\mathcal{H}_0(G)$ per questo grafo è di circa 1.5 Mbits. Un approccio alternativo basato sul precalcolo e la compressione (con Elias-Fano) delle risposte alla query Rank richiederebbe circa 2.2 Mbits. La nostra struttura succinta, utilizzando RLE per i dati associati, occupa circa 0.6 Mbits. Questo risultato evidenzia come la nostra struttura sia significativamente più compatta, superando persino il limite teorico $\mathcal{H}_0(G)$. Ciò è possibile perché la nostra rappresentazione è **lossy rispetto alla topologia completa** del grafo (non memorizza tutti gli archi), ma **lossless rispetto alla specifica query Rank** per cui è stata progettata.

## Slide 20/21: Future Direction: Bounded Query Time

Una considerazione importante per sviluppi futuri riguarda le prestazioni temporali delle query. Il tempo di risposta per un nodo implicito dipende dalla lunghezza della catena di successori designati percorsi per raggiungere un nodo esplicito. In DAG profondi, questo tempo può diventare significativo e variabile. Una possibile direzione è quella di garantire un **tempo di query limitato** (bounded).

[*click*]

Questo si può ottenere assicurando che ogni nodo implicito possa raggiungere un nodo esplicito entro un numero massimo $k$ di passi, eventualmente rendendo "espliciti" alcuni nodi intermedi selezionati strategicamente. La determinazione del minimo insieme di nodi da rendere espliciti è un problema computazionalmente complesso (legato al _minimum distance-k dominating set_), ma approcci euristici sono possibili. Si configura un trade-off tra garanzie temporali e spazio occupato.

## Slide 21/21: Thank You

Con questo si conclude la presentazione del mio lavoro. Vi ringrazio sentitamente per la vostra attenzione.
