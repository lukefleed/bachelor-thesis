## Slide 1/21: Titolo

Buongiorno a tutti. L'obiettivo di questa tesi è stato esplorare e sviluppare nuovi metodi per eseguire query sui cammini in grafi aciclici diretti pesati, con un focus particolare sull'efficienza in termini di spazio.

[ *pausa breve* ]

## Slide 2/21: Why Succinct?

Il contesto motivazionale è quello della gestione di dati su vasta scala. L'analisi efficace di questi dati richiede spesso un accesso rapido, il che favorirebbe la loro collocazione nella memoria RAM. Tuttavia per studiare questi dati (spesso già di grosse dimensioni) andiamo ad utilizzare strutture dati ausiliarie, come alberi o indici, che a loro volta richiedono uno spazio addizionale che spesso è ben superiore a quello dei dati stessi

[*click*]

Ci troviamo quindi di fronte a un classico trade-off: da un lato possiamo utilizzare le classiche tecniche di compressione, minimizzando lo spazio. Ma queste tipicamente impongono una fase di decompressione che preclude l'accesso diretto e rapido. Dall'altro lato, le strutture dati convenzionali offrono query veloci, ma a fronte di un considerevole overhead spaziale come abbiamo detto prima

[*click*]

In questo scenario, le **strutture dati succinte** rappresentano un paradigma promettente. Il loro scopo è duplice: rappresentare i dati impiegando uno spazio vicino al **minimo teorico** definito dalla teoria dell'informazione, e contemporaneamente supportare **query efficienti direttamente sulla rappresentazione compatta**, bypassando la necessità di una decompressione completa o parziale.

## Slide 3/21: Shannon Entropy

Per definire il concetto di "minimo teorico", introduciamo il concetto di'**Entropia di Shannon**. Questa misura quantifica l'incertezza media, o il contenuto informativo medio, associato a ciascun simbolo emesso da una sorgente di dati. L'entropia è intrinsecamente legata alla distribuzione di probabilità dei simboli generati: sorgenti più prevedibili hanno entropia minore, indicando una maggiore ridondanza e quindi un potenziale di compressione più elevato.

[*click*]

Il Source Coding Theorem di Shannon stabilisce che l'entropia rappresenta un **limite inferiore** che qualsiasi rappresentazione lossless di una sorgente deve rispettare. In altre parole, non è possibile comprimere i dati al di sotto di questo limite senza perdere informazioni

## Slide 4/21: Zero-Order Empirical Entropy

Nelle applicazioni reali, tuttavia, la distribuzione di probabilità della sorgente è raramente nota a priori. Disponiamo invece di una **sequenza finita** di dati osservati.

[*click*]

Per ottenere un benchmark pratico di comprimibilità in questo scenario, introduciamo l'**Entropia Empirica di Ordine Zero**. Anziché basarsi su probabilità teoriche, questa misura utilizza le **frequenze relative** con cui ciascun simbolo appare effettivamente all'interno della sequenza data.

[*click*]

Il prodotto tra la lunghezza della sequenza e la sua entropia empirica di ordine zero fornisce una stima concreta dello spazio minimo richiesto, basata esclusivamente sulle statistiche osservate nella sequenza stessa. Le strutture dati succinte spesso si prefiggono di raggiungere uno spazio vicino a questo valore empirico.

## Slide 5/21: Bitvectors and Fundamental Queries

Avendo delineato l'obiettivo, procediamo con la costruzione degli strumenti fondamentali, partendo dal caso più elementare: le sequenze binarie, o **bitvector**.

[*click*]

Su queste strutture, due operazioni primitive rivestono un'importanza cruciale: **rank** e **select**. L'operazione `rank` permette di contare le occorrenze di un bit specifico (0 o 1) fino a una data posizione.

[*click*]

Ad esempio, il rank degli 'uno' fino alla posizione 15 nel bitvector illustrato è 9. L'operazione `select`, invece, permette di localizzare la posizione della $j$-esima occorrenza di un bit specifico.

[*click*]

Ad esempio, la select del settimo 'uno' restituisce la posizione 12. Queste due operazioni sono inverse l'una dell'altra. Un obiettivo primario nella progettazione di strutture succinte è supportare rank e select in tempo computazionale costante, $O(1)$, utilizzando uno spazio addizionale (overhead) sublineare rispetto alla lunghezza della sequenza.

## Slide 6/21: RRR Structure: Entropy-Compressed Bitvectors

La richiesta di spazio $n+o(n)$ è già efficiente. Quello che ci chiediamo è se sia possibile fare di meglio quando il bitvector stesso presenta una struttura interna sfruttabile, come nel caso di sequenze molto sparse.

[*click*]

La risposta è si. Esistono strutture dati, come la **struttura RRR**, che comprimono il bitvector stesso, portando lo spazio occupato ad avvicinarsi al limite entropico $n H_0(B)$, calcolato sulla base delle frequenze dei bit 0 e 1 nel bitvector. Notevolmente, questa struttura preservano la capacità di eseguire le operazioni **rank e select in tempo costante**, $O(1)$.

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
