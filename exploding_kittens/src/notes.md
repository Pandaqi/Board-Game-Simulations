TO DO:
* Combo helper functies (identificeren + strategie checken)
* Combo uitvoering
* De code en strategieën voor kiezen van slachtoffer (en kaart vragen)
* Meer speelstrategieën
* Algemeen
  * Breng de "fixed strategy player 0" code terug
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
Willekeurig = gebruik combokaarten met 10% kans
Kans 0% (je doet er niks mee)
Kans 25%
Kans 50%
Kans 75%
Kans 100% (gebruik combo's meteen)
Geef de voorkeur aan combo's van drie

X NOTE: Hoe agressiever je Combo-strategie, hoe groter de kans dat je zo'n kaart probeert te _stelen_ van een ander.

== SPELEN == 

X Willekeurig.
X Speel alleen een kaart als je géén Defuse hebt (en dus niet "veilig" bent)
X Als de speler voor jou een Kitten heeft terug gestopt, ga ervan uit dat ie bovenaan ligt en speel een kaart om dit te omzeilen.
X Speel nooit een kaart
X Speel altijd een kaart.
X Speel altijd zoveel kaarten als je kan.

Spaar je kaarten tot het einde van het spel (2-3 spelers over, weinig kaarten)
Spaar je kaarten totdat je meerdere Aanvalskaarten tegelijk kan doen

Als je een "bekijk de stapel" kaart hebt, speel die altijd eerst
Als je een "bekijk de stapel" kaart hebt, speel die altijd eerst, maar alleen als je ook iets kan veranderen.


Spaar je kaarten op, zodat je in één keer alles op iemand anders kan gooien. (Vooral van toepassing op Attack kaarten.)

Als je een manier hebt om kaarten te stelen van een ander, gebruik die altijd.
Als je een manier hebt om kaarten te stelen van een ander, gebruik die nooit.
Als de vorige speler geen kaart heeft getrokken, probeer dan zelf ook geen kaart te trekken
Bluf: zelfs als je weet dat er geen Kitten aankomt, doe alsof je bang bent en deze moet omzeilen.
Probeer geen kaart te trekken als de kans op een Kitten boven een bepaald percentage uitkomt. (Kan je makkelijk uitrekenen, want je weet hoeveel Kittens erin zitten en mag de stapel tellen volgens de regels.)
Wees agressief aan het begin van het spel en defensief aan het einde
Wees defensief aan het begin van het spel en agressief aan het einde

== SLACHTOFFER KIEZEN ==
Willekeurig persoon en willekeurige kaart.
Willekeurig persoon en Defuse kaart.
Vraag altijd om een Defuse kaart. Kies de persoon die de meeste kans heeft deze te hebben. (Dit is publieke informatie als je een acceptabel geheugen hebt: wie al is ontploft of niet.)
Vraag om een kaart voor je combo's.
Vraag om een Nope kaart.
Kies één slachtoffer aan het begin en blijf die het hele spel lang kiezen.
Kies steeds een ander slachtoffer dan je vorige
Val de speler aan met de minste kaarten.
Val de speler aan met de meeste kaarten.
Val alleen mensen aan die eerst jou hebben aangevallen

== TERUG STOPPEN ==
X Willekeurig
X Bovenaan
X Tweede van boven
X Vierde van boven (om die kaart te ontwijken waarmee je kunt vooruit kijken)
X Bovenaan, maar alleen als de volgende speler weinig kaarten heeft. Anders willekeurig.