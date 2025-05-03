## Slide 1-2: Titolo + Table of Contents

Buongiorno a tutti. Oggi presento il lavoro dal titolo: _Efficient Succinct Data Structures on Directed Acyclic Graphs_.

[ *pausa breve* ]

## Slide 3: The memory bottleneck

Il contesto in cui ci muoviamo è dominato da un fenomeno ormai noto: l'**esplosione dei dati**. [ *Slide 3* ] In ambiti come la scienza, il web, o l'intelligenza artificiale, parliamo di volumi immensi, misurati in Petabyte ed Exabyte. Per poter analizzare efficacemente questa mole di informazioni, l'ideale sarebbe averle a disposizione nella memoria principale del computer, la RAM.

[ *Slide 4* ] Ma è proprio qui che incontriamo un **collo di bottiglia** significativo. La RAM, pur essendo estremamente **veloce**, ha una capacità limitata, tipicamente nell'ordine dei Gigabyte. Al contrario, i dischi o il cloud offrono capacità **enormi** (Terabyte, Petabyte), ma accedervi è molto più **lento** - parliamo di una differenza di prestazioni che può arrivare a un fattore di **centomila**!

[ *Slide 5* ] Inoltre, il problema non è limitato ai dati "grezzi". Molto spesso, le **strutture dati ausiliarie** – come indici o alberi – che costruiamo per interrogare efficientemente questi dati, finiscono per occupare uno spazio **considerevolmente maggiore** dei dati stessi. Di conseguenza, far rientrare sia i dati che le loro strutture ausiliarie nella veloce ma limitata RAM diventa una sfida notevole, a volte insormontabile.

## Slide 4: Succinct Data Structures

Questo ci pone di fronte a un **dilemma classico**, un trade-off tra due esigenze. [ *Slide 6* ]

- Da un lato, abbiamo la **compressione standard**: ci permette di raggiungere uno **spazio minimo**, ma tipicamente dobbiamo decomprimere i dati per poterli utilizzare o interrogare, perdendo l'accesso diretto.
- Dall'altro lato, le **strutture dati tradizionali** offrono **query veloci** e accesso diretto, ma spesso al costo di un **significativo ingombro** in memoria (il cosiddetto _space overhead_).

[ *Slide 7* ] È qui che intervengono le **strutture dati succinte**, con un obiettivo ambizioso: cercare di **combinare i vantaggi di entrambi gli approcci**. L'idea è duplice:

1.  Rappresentare i dati usando uno spazio che si avvicini il più possibile al **limite teorico minimo indispensabile** (un concetto legato all'entropia, che vedremo tra poco).
2.  Allo stesso tempo, permettere di eseguire **query efficienti direttamente su questa rappresentazione compatta**, senza bisogno di decomprimere tutto.

È un tentativo di ottenere, come si suol dire, "il meglio dei due mondi": compattezza e velocità.

## Slide 5: Shanon Entropy

Per capire fin dove possiamo spingerci con la compressione, abbiamo bisogno di un riferimento, un **limite teorico**. La domanda fondamentale è: qual è il numero **minimo** di bit, in media, necessari per codificare ogni simbolo proveniente da una certa sorgente di dati, senza perdere alcuna informazione?

[ *Slide 9* ] La teoria dell'informazione ci dà una risposta precisa con l'**Entropia di Shannon**, indicata come $H(X)$. La formula che vedete [ *indica la formula* ] quantifica l'**incertezza media**, o se volete, il **contenuto informativo medio**, associato a ogni simbolo generato dalla sorgente $X$. Questo valore dipende intrinsecamente dalle **probabilità reali** ($P_X$) con cui la sorgente produce ciascun simbolo. Si misura, tipicamente, in **bit per simbolo**.

[ *Slide 10* ] Il risultato fondamentale, noto come **Teorema di Codifica di Sorgente** di Shannon, stabilisce che l'entropia $H(X)$ non è solo una misura astratta, ma rappresenta il **limite teorico inferiore invalicabile** per la compressione lossless. Se conoscessimo le esatte probabilità della sorgente, nessun algoritmo potrebbe, in media, usare meno di $H(X)$ bit per rappresentare ogni simbolo generato.

## Slide 6: Zero-order entropy

C'è però un aspetto pratico: nella realtà, quasi mai **conosciamo le vere probabilità** $P_X$ della sorgente che ha generato i nostri dati. Quello che abbiamo è tipicamente una **sequenza specifica** di dati $S$. [ *Slide 11* ]

[ *Slide 12* ] Come possiamo allora stimare un limite di comprimibilità basato su ciò che osserviamo? Introduciamo l'**Entropia Empirica di Ordine Zero**, $H_0(S)$. L'idea è semplice: invece di usare le probabilità teoriche $P_X$, usiamo le **frequenze relative** con cui ciascun simbolo appare _effettivamente_ all'interno della nostra sequenza $S$. La formula [ *indica la formula* ] calcola l'entropia basandosi proprio su queste frequenze osservate ($n_s/n$).

[ *Slide 13* ] Qual è la sua rilevanza per noi? Il valore $n \times H_0(S)$ (la lunghezza della sequenza per la sua entropia empirica) ci fornisce un **benchmark pratico di spazio**. Le strutture dati succinte, come quella che presenteremo, spesso mirano a occupare uno spazio totale vicino a questo valore (o a sue varianti più raffinate, dette di ordine $k$) per la **specifica sequenza** $S$ che stiamo trattando. È il nostro punto di riferimento concreto, calcolato direttamente dai dati che abbiamo.

## Slide 7: Bitvector Rank & Select

Ora che abbiamo compreso l'obiettivo delle strutture dati succinte e il ruolo dell'entropia, iniziamo a costruire gli strumenti fondamentali. Partiamo dal caso più semplice: le sequenze binarie, o **bitvector**. Un bitvector $B$ è semplicemente una sequenza di $n$ zero e uno. [ *Indica Slide 15* ]

Su queste sequenze, ci interessano due operazioni query primitive ma potentissime: [ *Slide 16* ]

- **rank**, che conta quante volte un certo bit (0 o 1) compare fino a una data posizione $i$. È un'operazione di **conteggio**.
- **select**, che trova la posizione $j$ dell'$i$-esima occorrenza di un certo bit. È un'operazione di **localizzazione**.

[ *Slide 17* ] Ad esempio, `rank₁(15)` chiede quanti '1' ci sono nei primi 15 bit; la risposta è 9. [ *Slide 18* ] Invece, `select₁(7)` chiede dove si trova il settimo '1'; la risposta è alla posizione 12.

L'obiettivo cruciale per le strutture dati succinte è poter rispondere a queste query in tempo **costante**, $O(1)$, utilizzando uno spazio totale di $n + o(n)$ bit. Questo significa che, oltre agli $n$ bit per memorizzare il vettore stesso, l'informazione ausiliaria necessaria per le query veloci deve occupare uno spazio _sublineare_, che cresce più lentamente di $n$. Queste operazioni di Rank e Select sono veramente i "mattoni" su cui si costruiscono moltissime altre strutture succinte più avanzate.

## Slide 9: RRR

Abbiamo detto $n + o(n)$ bit. Ma se il bitvector è intrinsecamente "compressibile", ad esempio se contiene moltissimi zeri e pochissimi uno? Possiamo fare di meglio dello spazio lineare $n$?

[ *Slide 20* ] La risposta è sì. Esistono strutture, come la **struttura RRR** (proposta da Raman, Raman e Rao), che comprimono il bitvector stesso, avvicinandosi al **limite teorico dato dalla sua entropia empirica $n H_0(B)$**. La cosa notevole è che riescono a farlo **mantenendo il tempo costante $O(1)$** per le query di Rank e Select.

Il teorema [ *indica il teorema* ] formalizza questo: lo spazio richiesto è $B(n,m) + o(n)$ bit, dove $B(n,m)$ è l'informazione minima per specificare le posizioni degli $m$ uni (che è circa $n H_0(B)$), più un piccolo termine di overhead $o(n)$. Quindi, RRR adatta lo spazio alla "densità" del bitvector, offrendo compressione significativa per vettori sparsi, senza sacrificare la velocità delle query.

## Slide 10: WT

Finora abbiamo considerato solo l'alfabeto binario {0, 1}. Come possiamo estendere le potenti query Rank e Select a sequenze su alfabeti più grandi, come il testo o le sequenze biologiche? [ *Slide 21* ]

Una soluzione elegante è il **Wavelet Tree**. [ *Slide 22* ] Prendiamo come esempio la stringa "abracadabra". L'idea fondamentale del Wavelet Tree è **ridurre ricorsivamente il problema su un alfabeto grande a problemi su bitvector**.

Come funziona? [ *Descrivi Slide 23-25* ]

1.  Alla radice, **dividiamo l'alfabeto** corrente a metà (es. {a,b,c} | {d,r}).
2.  Costruiamo un **bitvector** della stessa lunghezza della sequenza: scriviamo '0' per i caratteri nella prima metà dell'alfabeto, '1' per quelli nella seconda (`001...`).
3.  Poi **ricorsivamente**: la sottosequenza dei caratteri mappati a '0' va al figlio sinistro, quella dei caratteri '1' al figlio destro.
4.  Su ogni figlio, **ripetiamo il processo**: dividiamo il suo (sotto)alfabeto e costruiamo il suo bitvector.
5.  Continuiamo fino ad arrivare alle foglie, che rappresentano i singoli caratteri.

Il risultato [ *Slide 25* ] è una **struttura ad albero binario bilanciata**, la cui altezza è circa il logaritmo della dimensione dell'alfabeto ($\log \sigma$). Ogni nodo interno contiene un bitvector con supporto per Rank e Select.

[*click*] Se rappresentiamo i bitvector con RRR, otteniamo un **Wavelet Tree Compresso**. Lo **spazio si adatta all'entropia** della sequenza originale $S$, diventando $n H_0(S) + o(n \log \sigma)$ bit. Questo è un grande vantaggio per sequenze comprimibili.Il **tempo** per le query fondamentali (`access`, `rank`, `select`) è $O(\log \sigma)$, perché tipicamente richiedono una discesa (o salita) lungo l'albero, eseguendo una query $O(1)$ su ogni bitvector incontrato

<!-- ## Slide 11: Wavelet Tree Performance

Quali sono le prestazioni di questa struttura?

[ *Slide 26* ] Per un **Wavelet Tree standard**, che usa bitvector $n+o(n)$:

- Lo **spazio** è circa $n \log \sigma$ bit (essenzialmente $n$ bit per livello, per $\log \sigma$ livelli), più l'overhead $o(n \log \sigma)$ per le strutture Rank/Select interne.
- Il **tempo** per le query fondamentali (`access`, `rank`, `select`) è $O(\log \sigma)$, perché tipicamente richiedono una discesa (o salita) lungo l'albero, eseguendo una query $O(1)$ su ogni bitvector incontrato.

[ *Slide 27* ] E se usiamo **bitvector compressi** (come RRR) nei nodi?

- Otteniamo un **Wavelet Tree Compresso**: lo **spazio si adatta all'entropia** della sequenza originale $S$, diventando $n H_0(S) + o(n \log \sigma)$ bit. Questo è un grande vantaggio per sequenze comprimibili.
- Il **tempo** di query rimane molto buono. `Access` è ancora $O(\log \sigma)$. `Rank` diventa $O(\log \sigma / \log \log n)$ e `Select` può essere $O(1)$ (a seconda dell'implementazione RRR).

In sintesi, i Wavelet Tree ci permettono di estendere Rank e Select ad alfabeti grandi, con la possibilità di ottenere anche compressione basata sull'entropia. -->

## Slide 12: Stringhe degeneri

Facciamo un ulteriore passo avanti. Abbiamo visto sequenze dove ogni posizione ha _un_ carattere. Ma cosa succede se in ogni posizione può esserci un **insieme di caratteri possibili**? [ *Slide 28* ]

Questa è una **stringa degenere**: una sequenza $X = X_1 \dots X_n$ dove ogni $X_i$ è un sottoinsieme dell'alfabeto $\Sigma$. Questo modello è utile, ad esempio, in bioinformatica per rappresentare incertezze o variazioni. Anche qui, possiamo definire le query analoghe: `subset-rank` (quanti insiemi fino a $i$ contengono $c$?) e `subset-select` (qual è l'indice del $j$-esimo insieme che contiene $c$?).

[ *Slide 29* ] Come supportarle efficientemente? Una tecnica efficace [ *Slide 30* ] è **ridurre il problema a operazioni standard** che già conosciamo. L'idea è costruire due strutture ausiliarie:

1.  Una **stringa normale $S$**, ottenuta concatenando tutti i caratteri di tutti gli insiemi $X_i$.
2.  Un **bitvector $R$**, che funge da "delimitatore", marcando con un '1' l'inizio della rappresentazione di ogni insieme $X_i$ all'interno di $S$.

Usando `rank` e `select` su queste due strutture $S$ e $R$, possiamo rispondere alle query `subset-rank` e `subset-select` sulla stringa degenere originale. Ad esempio, per `subset-rank`, troviamo prima la fine della rappresentazione di $X_i$ in $S$ usando `select` su $R$, e poi calcoliamo il `rank` normale del carattere $c$ sulla stringa $S$ fino a quel punto.

La complessità in termini di tempo dipenderà quindi esclusivamente dalle strutture dati succinte utilizzate per rappresentare S e R.

<!-- ## Slide 13: Performance

[ *Slide 31* ] Quindi, questa riduzione richiede una struttura dati per la stringa $S$ (come un Wavelet Tree) e una per il bitvector $R$ (come RRR).

[ *Slide 32* ] Qual è lo spazio totale? Se usiamo strutture succinte standard, lo spazio dipende principalmente da $N$, la somma totale delle dimensioni di tutti gli insiemi $X_i$. Diventa circa $N \log \sigma + N$ bit, più un termine $o(\cdot)$. È importante notare che dipende da $N$, non necessariamente dalla lunghezza $n$ della stringa degenere.

[ *Slide 33* ] È stato dimostrato che esiste anche un **limite inferiore** per questo problema: qualsiasi struttura deve usare almeno $N \log \sigma$ bit (meno un termine $o(\cdot)$) nel caso peggiore.

Il risultato notevole è che la nostra **riduzione**, implementata con strutture dati succinte standard, **si avvicina molto a questo limite inferiore teorico**, confermando la sua efficienza in termini di spazio. -->

## Slide 14: Table of Contents

[ *Qui si fa la transizione alla sezione successiva* ] Ok, abbiamo visto come estendere Rank e Select a bitvector, alfabeti generici e persino a queste stringhe "degeneri". Ora, faremo un passo ulteriore e vedremo come il problema delle stringhe degeneri possa essere visto sotto una luce diversa, quella dei **grafi**, che ci porterà al contributo principale di questa tesi.

## Slide 15: Da stringhe degeneri a DAG

Finora, la nostra analisi si è concentrata sulle sequenze. Adesso, introduciamo un cambio di prospettiva significativo, mostrando come un problema legato alle sequenze – specificamente, quello delle **stringhe degeneri** che abbiamo appena discusso – possa essere efficacemente rimodellato nel contesto dei **grafi**.

Ricorderete che una stringa degenere permette a ciascuna posizione di rappresentare un insieme di caratteri. [ *Indica esempio Slide 35* ] L'**idea centrale** [ *Slide 36* ] è che le interrogazioni su tale stringa (ad esempio, il conteggio di un carattere specifico 'c') possono essere tradotte nella costruzione e nell'analisi di un **grafo aciclico diretto (DAG) pesato**, costruito ad hoc.

Vediamo come avviene questa costruzione: [ *Gesticola verso Slide 37-39* ]

1.  Si definisce un insieme di **vertici**: un nodo sorgente unico $s$, e un vertice $v_{k,a}$ per ogni carattere $a$ presente nell'insieme $X_k$ alla posizione $k$.
2.  Si assegna un **peso** $w_c$ a ciascun vertice: $w_c(s)=0$; per gli altri vertici $v_{k,a}$, il peso è 1 se $a$ coincide con il carattere target $c$, e 0 altrimenti. Questo serve a marcare le occorrenze del carattere di interesse.
3.  Infine, si definiscono gli **archi**: dalla sorgente $s$ a tutti i vertici del primo livello ($k=1$); e poi, da ogni vertice $v_{k,a}$ a _tutti_ i vertici $v_{k+1,b}$ del livello successivo. Questi archi modellano la progressione sequenziale.

La struttura risultante è intrinsecamente aciclica, un **DAG**, data la direzionalità degli archi tra livelli successivi.

## Slide 15: Esempio DAG

Visualizziamo questa costruzione con il nostro esempio, impostando il carattere target $c = \texttt{'A'}$.

[ *Descrivi progressivamente le slide 40-43* ]
Partiamo dalla sorgente $s$. [ *Slide 40* ] Vengono creati archi verso i nodi corrispondenti ai caratteri in $X_1=\{\texttt{A,C,G}\}$. Il nodo $(1,\texttt{A})$ riceve peso 1 (giallo), gli altri peso 0 (grigio). [ *Slide 41* ] Successivamente, si collegano tutti i nodi del livello 1 ai nodi del livello 2, derivati da $X_2=\{\texttt{A,T}\}$; il nodo $(2,\texttt{A})$ ha peso 1. [ *Slide 42* ] Il processo si ripete per il livello 3 ($X_3=\{\texttt{T,C,A}\}$), assegnando peso 1 a $(3,\texttt{A})$. [ *Slide 43* ] E infine per il livello 4 ($X_4=\{\texttt{A,G}\}$), ottenendo la struttura DAG completa $G_A$.

L'osservazione cruciale è la seguente: **ogni cammino** dalla sorgente $s$ a un vertice al livello $k$ corrisponde biunivocamente a una delle possibili sequenze di $k$ caratteri realizzabili dalla stringa degenere. Inoltre, per costruzione, la **somma dei pesi $w_A$ lungo tale cammino conta precisamente il numero di occorrenze del carattere 'A'** in quella specifica sequenza.

Questo dimostra come una query sulla stringa degenere sia riconducibile a una query relativa ai **pesi cumulativi dei cammini** nel DAG associato. Tale constatazione motiva lo studio di strutture dati efficienti per gestire proprio questo tipo di interrogazioni su DAG pesati generici, che costituisce il nucleo di questa tesi.

## Slide 16: Path Weight Queries

Ora, concentriamoci su un DAG pesato generico. Come possiamo tenere traccia di **tutti i possibili pesi totali** dei cammini che partono dalla sorgente e arrivano a un certo nodo $v$?

[ *Indica definizione Slide 44* ]

Consideriamo ora un DAG pesato generico $G=(V,E,w)$. Come possiamo caratterizzare formalmente l'insieme di **tutti i possibili pesi cumulativi** dei cammini che originano dalla sorgente $s$ e terminano in un dato vertice $v$?

[ *Indica definizione Slide 44* ]

A tal fine, introduciamo l'**$\mathcal{O}$-set**, denotato $\mathcal{O}_v$. Questo insieme contiene, per definizione, tutti e soli i distinti valori di peso $W(P)$ per i cammini $P$ da $s$ a $v$. La sua costruzione è ricorsiva:

- Per il nodo sorgente $s$, l'$\mathcal{O}$-set è $\mathcal{O}_s = \{0\}$.
- Per qualsiasi altro nodo $v$, il suo $\mathcal{O}$-set, $\mathcal{O}_v$, si ottiene considerando **tutti i suoi predecessori** $u \in Pred(v)$. Per ciascun predecessore $u$, si prendono tutti gli elementi $y$ del suo $\mathcal{O}$-set, $\mathcal{O}_u$, e si calcola $y + w(v)$. L'$\mathcal{O}$-set finale di $v$ è l'**unione** di tutti questi valori $y+w(v)$ così generati, mantenendo solo i valori **distinti** e considerandoli ordinati.

In essenza, $\mathcal{O}_v$ rappresenta l'aggregazione completa delle informazioni sui pesi dei cammini che raggiungono $v$.

## Slide 17: Path Weight Example

Illustriamo questo processo di costruzione con l'esempio del grafo in figura, dove il peso di ciascun nodo coincide con il suo identificativo.

[ *Percorri le slide/animazioni sinteticamente* ]
Si parte da $\mathcal{O}_0=\{0\}$. [ *Slide 45* ] Poi si calcolano $\mathcal{O}_1=\{1\}$ e $\mathcal{O}_3=\{3\}$. [ *Slide 46* ] Per il nodo 7, che ha peso 7 e unico predecessore 1, si ha $\mathcal{O}_7=\{1+7\}=\{8\}$. [ *Slide 47* ] Per il nodo 6 (peso 6, predecessori 1 e 3), si uniscono $\{1+6\}$ e $\{3+6\}$, ottenendo $\mathcal{O}_6=\{7, 9\}$. [ *Slide 48* ] Si procede analogamente per gli altri nodi [ *menziona brevemente 2, 9, 10, 5* ] ...fino a calcolare $\mathcal{O}_8$ unendo i contributi (shiftati di $w(8)=8$) provenienti da $\mathcal{O}_5$, $\mathcal{O}_9$ e $\mathcal{O}_{10}$, e mantenendo i valori distinti. [ *Slide 53* ]

Questo processo iterativo, seguendo un ordine topologico, permette di determinare l'insieme completo dei pesi di cammino per ogni vertice.

## Slide 18: Rank

Definiamo ora la **query centrale** di questo lavoro, la query **Rank** sul nodo $N$, denotata $\mathrm{rank}_G(N)$.

Mentre l'$\mathcal{O}$-set $\mathcal{O}_N$ cattura i pesi _finali_ dei cammini, la query $\mathrm{rank}_G(N)$ mira a descrivere l'insieme dei valori di peso cumulativo che possono essere considerati "attivi" _durante_ la fase di elaborazione associata al nodo $N$ stesso (il cui contributo è $w(N)$).

La sua definizione formale è la seguente: [ *Indica Slide 54* ]

1.  Per ogni peso finale $x$ nell'$\mathcal{O}$-set di $N$, si genera l'intervallo di valori $[\max(0, x - w(N) + 1), x]$. Questo intervallo rappresenta i valori cumulativi nel momento in cui si sta sommando il peso $w(N)$ per raggiungere $x$.
2.  Si calcola l'**unione** $S_N$ di tutti questi intervalli generati.

[ *Indica Slide 55* ] Il risultato restituito dalla query $\mathrm{rank}_G(N)$ è la **rappresentazione canonica** di $S_N$ come **collezione minima di intervalli interi chiusi e disgiunti**, ottenuta fondendo gli intervalli iniziali che si sovrappongono o sono adiacenti.

## Slide 19: The challenge

Abbiamo definito gli $\mathcal{O}$-set e la query Rank. Tuttavia, emerge un problema pratico cruciale: [ *Slide 56* ] la cardinalità degli $\mathcal{O}$-set, $|\mathcal{O}_v|$, può crescere in modo **esponenziale** in alcuni DAG. Una memorizzazione esplicita di tutti gli $\mathcal{O}$-set diventa quindi impraticabile.

Come superare questo ostacolo? [ *Slide 57-58* ] La nostra strategia si fonda su due principi: **Partizionamento** dei nodi e **Indirezione** per la rappresentazione dell'informazione.

**Partizionamento** [ *Slide 57* ]: Classifichiamo i vertici in:

1.  **Nodi Espliciti ($V_E$)**: Tipicamente i nodi sink (senza successori). Per questi nodi, **memorizziamo direttamente** il loro $\mathcal{O}$-set $\mathcal{O}_v$.
2.  **Nodi Impliciti ($V_I$)**: Tutti gli altri vertici. Per questi, **evitiamo la memorizzazione diretta** dell'$\mathcal{O}$-set $\mathcal{O}_v$, che sarebbe troppo costosa.

**Indirezione** [ *Slide 58* ]: Per un nodo implicito $v$, l'informazione sull'$\mathcal{O}$-set viene ricostruita dinamicamente tramite due elementi memorizzati:

1.  Un puntatore a un **Successore Designato**, $\sigma(v) \in Succ(v)$, scelto tramite un'euristica che vedremo.
2.  Una **Sequenza di Offset**, $\mathcal{I}_v$, che codifica la relazione tra gli elementi di $\mathcal{O}_v$ e quelli dell'$\mathcal{O}$-set del successore designato $\mathcal{O}_{\sigma(v)}$.

In sintesi, deleghiamo la memorizzazione completa solo a un sottoinsieme di nodi (gli Espliciti) e implementiamo un meccanismo di riferimento e trasformazione per tutti gli altri (gli Impliciti).

## Slide 20: Implicit Reconstruction

Approfondiamo il meccanismo di indirezione per i nodi impliciti $v \in V_I$.

**1. Scelta del Successore Designato $\sigma(v)$:**
Dato un nodo $v$, potremmo avere più successori. Quale scegliamo come $\sigma(v)$? Applichiamo un'**euristica**: selezioniamo il successore $u \in Succ(v)$ che possiede l'$\mathcal{O}$-set di **cardinalità minima**, $|\mathcal{O}_u|$. [ *Indica formula argmin* ] A parità di cardinalità, si sceglie l'ID minimo. L'intuizione è scegliere la direzione che sembra portare a una minore complessità informativa.

**2. Ruolo della Sequenza di Offset $\mathcal{I}_v$:**
Esiste una relazione matematica precisa tra gli elementi dell'$\mathcal{O}$-set di $v$ (chiamiamoli $x$) e quelli dell'$\mathcal{O}$-set del suo successore designato $u=\sigma(v)$ (chiamiamoli $y$): ogni $x \in \mathcal{O}_v$ deriva da un $y \in \mathcal{O}_u$ tramite la formula $x = y - w(u)$. La sequenza di offset $\mathcal{I}_v = (j_0, j_1, \dots, j_{m-1})$, dove $m=|\mathcal{O}_v|$, memorizza esattamente questa corrispondenza: per l'$k$-esimo elemento $x_k$ di $\mathcal{O}_v$, l'offset $j_k$ memorizzato in $\mathcal{I}_v[k]$ indica l'**indice** dell'elemento $y_{j_k}$ corrispondente nell'$\mathcal{O}$-set del successore $u$.

Quindi, $\mathcal{I}_v$ funziona come una "tabella di traduzione": per recuperare il $k$-esimo valore di $\mathcal{O}_v$, usiamo $\mathcal{I}_v[k]$ per sapere quale indice $j_k$ interrogare nell'$\mathcal{O}$-set del successore $\sigma(v)$, per poi applicare la correzione sottraendo il peso $w(\sigma(v))$.

## Slide 21: Example

Visualizziamo ora l'applicazione di questa strategia all'esempio precedente.

[ *Slide 60: Struttura statica* ]
Osserviamo la partizione: il nodo 8 è l'unico **nodo esplicito** (verde), poiché è un sink, e quindi memorizza direttamente il suo $\mathcal{O}_8$. Tutti gli altri nodi sono **impliciti** (bianchi/rosso sorgente) e memorizzano ciascuno la propria sequenza di offset $\mathcal{I}_v$. Le frecce grigie rappresentano la topologia originale del DAG.

[ *Slide 61: Successori designati* ]
Le **frecce blu** evidenziano il **successore designato $\sigma(v)$** scelto per ogni nodo implicito, seguendo l'euristica di minimizzare la cardinalità dell'$\mathcal{O}$-set del successore. Ad esempio, $\sigma(7)=9$ poiché $|\mathcal{O}_9|=3 < |\mathcal{O}_5|=4$. In caso di parità, come per il nodo 6 (successori 2 e 9, entrambi con $|\mathcal{O}|=3$), si sceglie l'ID minore, quindi $\sigma(6)=2$.

L'intera rappresentazione succinta consiste quindi nell'array dei pesi, nell'array dei successori $\sigma$ (con marcatori per i nodi espliciti), e nella collezione dei dati associati $\mathcal{D}$ (che contengono $\mathcal{O}$-set o sequenze $\mathcal{I}$).

## Slide 22: Esempio dettaglio

Illustriamo ora il processo di query, ad esempio, per determinare il **primo elemento (indice 0) dell'$\mathcal{O}$-set del nodo 7**, $\mathcal{O}_7[0]$. Poiché 7 è un nodo implicito, dobbiamo ricostruire questo valore.

[ *Slide 62* ] Iniziamo dal nodo 7, con indice target $k=0$ e somma pesi accumulata $sum=0$. Seguiamo il successore designato $\sigma(7)=9$.

[ *Slide 63* ] Consultiamo l'offset $\mathcal{I}_7=(1)$. L'elemento all'indice $k=0$ è $1$. Questo diventa il nuovo indice target per il nodo 9. Accumuliamo il peso del successore: $sum = 0 + w(9) = 9$.

[ *Slide 64* ] Dal nodo 9 (indice target $k=1$), seguiamo $\sigma(9)=5$. Consultiamo $\mathcal{I}_9=(1, 2, 3)$. L'elemento all'indice $k=1$ è $2$. Aggiorniamo l'indice target a $k=2$ per il nodo 5. Accumuliamo il peso: $sum = 9 + w(5) = 14$.

[ *Slide 65* ] Dal nodo 5 (indice target $k=2$), seguiamo $\sigma(5)=8$. Consultiamo $\mathcal{I}_5=(0, 6, 7, 8)$. L'elemento all'indice $k=2$ è $7$. Aggiorniamo l'indice target a $k=7$ per il nodo 8. Accumuliamo il peso: $sum = 14 + w(8) = 22$.

Siamo arrivati al nodo 8, che è **esplicito**. La procedura di inseguimento termina. Recuperiamo il valore all'indice target $k=7$ dall'$\mathcal{O}$-set memorizzato $\mathcal{O}_8$: $\mathcal{O}_8[7] = 30$.
Il valore originale $\mathcal{O}_7[0]$ si ottiene sottraendo la somma dei pesi accumulati: $30 - sum = 30 - 22 = 8$.

Questo processo illustra come, seguendo la catena dei successori designati e utilizzando gli offset, sia possibile ricostruire qualsiasi elemento dell'$\mathcal{O}$-set di un nodo implicito, facendo riferimento infine ai dati memorizzati nei nodi espliciti.

## Slide 23: Componenti

Quindi, la nostra struttura dati per il DAG è composta essenzialmente da tre parti principali, che possiamo immaginare come degli array indicizzati dall'ID del nodo. [ *Slide 66* ] Per ogni nodo $i$, memorizziamo:

1.  Il suo **peso**, $w(i)$.
2.  L'ID del suo **successore designato**, $\sigma(i)$ (o un marcatore speciale se è un nodo esplicito/sink).
3.  I **Dati Associati**, $\mathcal{D}[i]$, che sono la parte più interessante: contengono o l'$\mathcal{O}$-set completo (se $i$ è esplicito) o la sequenza di offset $\mathcal{I}_i$ (se $i$ è implicito).

[ *Slide 67* ] Possiamo vedere l'intera struttura come un insieme di array paralleli: un array per i pesi ($\mathcal{W}$), uno per i successori ($\Sigma$), e una collezione di sequenze per i dati associati ($\mathcal{D}$). Questo layout, chiamato "Struct of Arrays", è spesso buono per le prestazioni della cache e ci permette di pensare a come comprimere ciascun componente separatamente.

## Slide 24: Compression

Ora, come possiamo comprimere questi componenti per ridurre ulteriormente lo spazio?

[ *Slide 68* ]

- L'array dei **Pesi** ($\mathcal{W}$) e quello dei **Successori** ($\Sigma$) sono semplicemente sequenze di numeri interi positivi.

[ *Slide 69* ]

- Per questi, possiamo usare tecniche di **codifica intera a lunghezza variabile**. Sono metodi classici che usano meno bit per i numeri piccoli e più bit per i numeri grandi. [ *Menziona brevemente il link alla libreria Rust se rilevante e permesso* ]
- Un'alternativa, se i pesi hanno un range limitato, sono i **Wavelet Trees** che abbiamo visto prima.

[ *Slide 70* ]

- La parte più complessa sono i **Dati Associati** ($\mathcal{D}$). Qui abbiamo delle sequenze di interi (gli $\mathcal{O}$-set per i nodi espliciti, gli offset $\mathcal{I}_v$ per gli impliciti) che hanno una proprietà importante: sono **strettamente crescenti** (monotoniche).

[ *Slide 71* ]

- Per sequenze monotone, abbiamo opzioni specifiche molto efficienti:
  - L'**Elias-Fano Encoding**: una tecnica ottimale in teoria per rappresentare sequenze monotone in poco spazio, supportando accessi veloci.
  - Il **Run-Length Encoding (RLE)**: è particolarmente efficace se la sequenza contiene lunghe "corse" di numeri consecutivi (es. 10, 11, 12, 13...). Invece di memorizzare ogni numero, memorizza l'inizio della corsa e la sua lunghezza.

La scelta tra Elias-Fano e RLE dipende da come sono fatte le sequenze $\mathcal{O}_v$ e $\mathcal{I}_v$ nel grafo specifico: se ci sono molte "corse", RLE può essere migliore, altrimenti Elias-Fano è una scelta solida.

## Slide 25: Entropia

Abbiamo costruito la nostra struttura dati. Ma quanto è efficiente in termini di spazio? Per capirlo, ci serve un **termine di paragone**, una baseline.

Introduciamo l'**Entropia di Ordine Zero del Grafo**, $\mathcal{H}_0(G)$. Questa misura rappresenta un **limite inferiore teorico** per lo spazio necessario a memorizzare l'_intero_ grafo pesato (nodi, archi e pesi) senza perdere alcuna informazione (lossless).

Si calcola sommando due contributi: il costo per memorizzare i **pesi** ($\mathcal{H}_W(G)$) e il costo per memorizzare la **topologia**, cioè gli **archi** ($\mathcal{H}_E(G)$).

[ *Slide 73* ]
Il costo per i **pesi**, $\mathcal{H}_W(G)$, è semplicemente la somma dei bit minimi necessari per rappresentare ciascun peso individualmente.

[ *Slide 74* ]
Il costo per la **topologia**, $\mathcal{H}_E(G)$, è legato a quanti modi ci sono per scegliere gli $m$ archi presenti nel grafo tra tutti i possibili $n(n-1)$ archi in un grafo con $n$ nodi. È un termine combinatorio ($\log \binom{n(n-1)}{m}$) che dipende da quanti nodi e archi ci sono.

[ *Slide 75* ]
Quindi, $\mathcal{H}_0(G)$ è la somma di questi due. **Qualsiasi metodo che salva l'intero grafo (pesi e tutti gli archi) ha bisogno di almeno $\mathcal{H}_0(G)$ bit.**

## Slide 26: Space Complexity

Ora confrontiamo lo spazio della nostra struttura con queste baseline, usando un esempio reale da una rete Bitcoin (con circa 22mila nodi e 50mila archi).

[ *Slide 76: Tabella - Solo baseline* ]
Vediamo i numeri:

- L'entropia $\mathcal{H}_0(G)$ per questo grafo è circa **1.5 Milioni di bit**. La maggior parte è dovuta alla topologia (gli archi).

[ *Slide 77: Tabella - Aggiungi Precomputation* ]

- Se provassimo a precalcolare e memorizzare le risposte a tutte le possibili query Rank:
  - Memorizzandole esplicitamente servirebbero quasi **5 Milioni di bit**.
  - Comprimendole con Elias-Fano (una tecnica buona per sequenze monotone come i risultati delle query), scendiamo a **2.2 Milioni di bit**. Comunque tanto.

[ *Slide 78: Tabella - Aggiungi la nostra struttura* ]

- E la nostra **struttura succinta per DAG** (usando RLE per comprimere gli offset)? Stima totale: circa **600 mila bit**!

[ *Slide 79: Tabella - Risultato chiave* ]
Il **risultato chiave** è evidente:

- La nostra struttura è **2.5 volte più piccola** dell'entropia $\mathcal{H}_0(G)$ (il limite teorico per salvare _tutto_ il grafo).
- Ed è quasi **4 volte più piccola** dell'approccio con precalcolo compresso (Elias-Fano).

## Slide 27: How?

Ma com'è possibile che la nostra struttura usi _meno_ spazio dell'entropia $\mathcal{H}_0(G)$, che era un limite inferiore?

[ *Mostra schema comparativo* ]

La risposta sta in cosa memorizziamo:

- Un encoding lossless del grafo (che deve raggiungere $\mathcal{H}_0(G)$) memorizza **tutto**: Nodi, Pesi e **tutti gli Archi**.
- La nostra struttura succinta memorizza i Pesi ($\mathcal{W}$), i puntatori ai Successori ($\Sigma$), e i Dati Associati ($\mathcal{D}$ = O-set/Offset).

La **differenza fondamentale** è che la nostra struttura è **lossy rispetto alla topologia completa del grafo**:

- **Non memorizza tutti gli archi $E$**. Non possiamo ricostruire l'intero insieme di archi originali.
- Memorizza solo il **successore scelto $\sigma(v)$** per ogni nodo implicito, non tutti i suoi successori reali.

Tuttavia, la struttura è **lossless specificamente per calcolare la query Rank$_G$** che ci interessa. Abbiamo scartato informazioni sulla topologia generale, che non erano strettamente necessarie per la nostra query specifica, ottenendo così uno spazio inferiore a $\mathcal{H}_0(G)$.

## Slide 28: Conclusione

In questa tesi, siamo partiti dai fondamenti delle strutture dati succinte, capendo perché sono importanti per gestire la mole di dati odierna e vedendo come funzionano su sequenze semplici e complesse.

Il cuore del lavoro è stata l'introduzione di una **nuova strategia per rappresentare DAG pesati in modo succinto**, focalizzata sul supporto efficiente di una specifica query, la query **Rank$_G$**, che aggrega informazioni sui pesi dei cammini.

Abbiamo visto come, partizionando i nodi in **Espliciti** (che memorizzano le informazioni complete) e **Impliciti** (che usano puntatori a successori designati e sequenze di offset), possiamo **ricostruire le informazioni necessarie** per la query senza dover memorizzare le enormi liste di pesi ($\mathcal{O}$-set) per tutti i nodi.

Abbiamo anche discusso come **comprimere** ulteriormente i componenti della nostra struttura usando tecniche come codifica intera, Elias-Fano e Run-Length Encoding.

Un risultato chiave è che la nostra struttura, essendo **ottimizzata per la query Rank$_G$**, può occupare **meno spazio** rispetto al limite teorico ($\mathcal{H}_0(G)$) necessario per memorizzare l'intero grafo senza perdite. Questo è possibile perché **sacrifichiamo informazioni sulla topologia completa** del grafo (non memorizziamo tutti gli archi), che non sono indispensabili per la nostra specifica query. Abbiamo visto nell'esempio che questo porta a un risparmio di spazio significativo rispetto sia all'entropia del grafo sia ad approcci basati sul precalcolo delle query.

[ *Pausa prima di passare ai passi futuri* ]

[ *Slide 82* ]
C'è però un aspetto da considerare per il futuro: le **prestazioni delle query**. Il tempo per rispondere a una query su un nodo implicito dipende da quanto è lunga la **catena di successori** che dobbiamo seguire per arrivare a un nodo esplicito. In grafi molto "profondi", questa catena potrebbe diventare lunga, rendendo il tempo di query variabile o lento nel caso peggiore.

[ *Slide 83* ]
Come possiamo migliorare? L'idea è **garantire un tempo di query limitato**.

- **Soluzione**: Assicuriamoci che ogni nodo implicito possa raggiungere un nodo esplicito seguendo al massimo un numero fisso $k$ di passi lungo la catena dei successori. Questo significa rendere "espliciti" alcuni nodi che prima erano impliciti.
- **Sfide**: Scegliere il **minor numero possibile** di nodi da rendere espliciti per soddisfare questa condizione è un problema difficile (NP-hard), simile a trovare un "minimum distance-k dominating set". Servirebbero buone euristiche.
- **Trade-off**: Rendere più nodi espliciti porta a **query più veloci e prevedibili**, ma aumenta lo **spazio** totale della struttura (perché dobbiamo memorizzare più $\mathcal{O}$-set completi). Bisogna trovare il giusto bilanciamento tra velocità garantita e compattezza.

Questo apre direzioni interessanti per ottimizzare ulteriormente la struttura.

[ *Passa alla Slide 84* ]

Con questo, concludo la mia presentazione. Vi ringrazio per la vostra attenzione.
