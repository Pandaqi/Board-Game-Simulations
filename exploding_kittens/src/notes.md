TO DO:
* TESTS voor de COMBO functies
* Maak de default dat je alleen Future speelt als je het kan veranderen => maar dat zou de volgorde weer moeilijk maken, want dan wil je _eerst_ rondvragen en een anti-kitten kaart spelen
  * Sowieso slimmere Future strategieën. Heb nu het idee dat spelers te veel kaarten eraan gaan opbranden omdat ze mijn regels willen volgen tot het UITERSTE
  * Dat moet sowieso opgelost. Ik denk dat de andere Play strategieën hierin leidend zijn: wat is jouw maximum aan kaarten voordat je "opgeeft" voor deze beurt?
  * De Play strategie moet denk ik LEIDEND zijn. (Dus als Play strategie zegt: nooit kaarten spelen, dan overschrijft dat de Future en Anti dingen.) 
* NOPE:
  * Betere definitie "direct attack" toepassen + strategie/voorkeur om combo's te nopen (meer waarde voor je nope)
  * Toestaan dat je kaarten stelen nopet 
  * Splits in vier: kans op gebruiken/vragen, eventuele override, verdedigend nopen (in welke situatie gebruik je een Nope als je direct wordt aangevallen?), multinope
* ATTACK: track hoeveel attacks er op elkaar zijn gespeeld in de state? (Eerste attack = repeating turns naar 1, daarna is het repeat_turns + num_attacks*2, wordt gereset zodra speler wisselt aan einde loop)
* Meer speelstrategieën
* GEBRUIK de andere soorten plots (van spelverloop) om te testen of de simulatie wel fatsoenlijk werkt
* Algemeen
  * Breng de "fixed strategy player 0" code terug
  * Bijhouden welke _beginkaarten_ de winnende speler had, misschien is dat interessante data?
  * Gebruik de debugger op iets meer plekken
  * Kan ik nog andere getalletjes bijhouden die interessant zijn? Zoals "kans dat je wint als jij een tweede defuse kaart weet te trekken" of "kans dat je wint afhankelijk van met hoeveel Nope kaarten je begint"
  * Vind een manier om het verloop van een potje in een grafiek of tabel te zetten? (Is mooi én goede check)

Dat kan zo ongeveer:

data = <lijst met hoeveel kaarten elke speler in deze ronde had>
offset = afstand tussen punten
ctx.draw_series(LineSeries::new(
    (0..).zip(data.iter()).map(|(x, y)| (x, *y + offset)),
    &BLUE,
))
.unwrap();


== NOPE ==
X Willekeurig = Kans 10% (there are many occasions for noping, so a lower value is more logical)
X Kans 0%
X Kans 25%
X Kans 50%
X Kans 75%
X Kans 100%
X Alleen als ze zelf meer dan genoeg kaarten hebben
X Alleen als ze zelf géén Defuse meer hebben
X Nope alles dat jou direct aanvalt
X Nope alles dat jou direct aanvalt, als je geen Defuse meer hebt
X Spaar ze tot het einde (slechts 2 spelers over), gebruik ze dan op alles
X Gebruik ze alleen om een bestaande Nope meteen te Ontnopen
X Gebruik ze alleen om een bestaande Nope te ontnopen, als dat een directe aanval op jou is

X NOTE: Hoe agressiever je Nope-strategie, hoe groter de kans dat je die kaart probeert te _stelen_ van een ander.

== COMBO ==
X Willekeurig = gebruik combokaarten met 10% kans
X Kans 0% (je doet er niks mee)
X Kans 25%
X Kans 50%
X Kans 75%
X Kans 100% (gebruik combo's meteen)
X Geef de voorkeur aan combo's van drie, soms
X Geef de voorkeur aan combo's van drie, altijd
X Gebruik alle kaarten (niet alleen katkaarten) voor combo's
X Gebruik alle kaarten (niet alleen katkaarten) voor combo's van drie

X NOTE: Hoe agressiever je Combo-strategie, hoe groter de kans dat je zo'n kaart probeert te _stelen_ van een ander.

== SPELEN == 

X Willekeurig.
X Speel alleen een kaart als je géén Defuse hebt (en dus niet "veilig" bent)
X Als de speler voor jou een Kitten heeft terug gestopt, ga ervan uit dat ie bovenaan ligt en speel een kaart om dit te omzeilen.
X Speel nooit een kaart
X Speel altijd een kaart.
X Speel altijd zoveel kaarten als je kan.

TO DO: Als je weet dat er een Kitten aankomt, eerst proberen om met Combo's/Favor kaarten te verkrijgen die misschien helpen => Hoe integreer je dit makkelijk? Is dit dan hetzelfde voor alle spelers?

Spaar je kaarten tot het einde van het spel (2-3 spelers over, weinig kaarten)
Spaar je Attack en Skip kaarten tot het einde

Probeer geen kaart te trekken als de kans op een Kitten boven een bepaald percentage uitkomt. (Kan je makkelijk uitrekenen, want je weet hoeveel Kittens erin zitten en mag de stapel tellen volgens de regels.) => Ik kan daarvoor gewoon het deck tellen

Speel altijd "bekijk de stapel"-kaart als eerste, mits je iets kan veranderen.
Speel alleen "bekijk de stapel"-kaarten als je onveilig bent (geen Defuse meer)

Spaar je kaarten op, zodat je in één keer alles op iemand anders kan gooien. (Vooral van toepassing op Attack kaarten.)

Als je een manier hebt om kaarten te stelen van een ander, gebruik die altijd.
Als je een manier hebt om kaarten te stelen van een ander, gebruik die nooit.

Copycat: Als de vorige speler geen kaart heeft getrokken, probeer dan zelf ook geen kaart te trekken
Bluf: als je "see the future" aangeeft dat er géén kitten aankomt, speel alsnog een kaart alsof dat wél zo is.
StartHoard: begin het spel met zoveel mogelijk kaarten verzamelen, en dan halverwege stop je volledig en reageert alleen als nodig

== SLACHTOFFER KIEZEN ==
X Willekeurig persoon en willekeurige kaart.
X Willekeurig persoon en Defuse kaart.
X Vraag altijd om een Defuse kaart. Kies de persoon die de meeste kans heeft deze te hebben. (Dit is publieke informatie als je een acceptabel geheugen hebt: wie al is ontploft of niet.)
X Val de speler aan met de minste kaarten.
X Val de speler aan met de meeste kaarten.
X Val de speler vóór jou aan.
X Val de speler ná jou aan.
X Kies één slachtoffer aan het begin en blijf die het hele spel lang kiezen.
X Kies steeds een ander slachtoffer dan je vorige

== TERUG STOPPEN ==
X Willekeurig
X Bovenaan
X Tweede van boven
X Vierde van boven (om die kaart te ontwijken waarmee je kunt vooruit kijken)
X Bovenaan, maar alleen als de volgende speler weinig kaarten heeft. Anders willekeurig.