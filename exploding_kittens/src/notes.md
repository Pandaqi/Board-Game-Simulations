== TOEKOMSTIGE TO DO ==
* Strategieën
  * Veel meer (gedetailleerde) speelstrategieën
  * Future nog eens bekijken?
  * Toch inbouwen dat, als je zéker bent dat een speler nog een Defuse hebt, je jouw strategie overschrijdt en die kaart vraagt (als je een combo van 3 hebt)??
* Kijk nog eens naar hoe Attack is ingebouwd en of dat nou 100% klopt met de spelregels.
* Misschien nog wat meer getalletjes en grafieken. Ik bedoel, als we de simulatie toch hebben ...
* Schrijf TESTS voor de COMBO functies en meer specifieke strategieën

Zo kan je ongeveer een lijnplot maken (meerdere lijnen, verloop van potje over tijd)

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
X Spaar je kaarten tot het einde van het spel, word dan agressief
X Wees agressief aan het begin van het spel, spaar je kaarten aan het einde

Probeer geen kaart te trekken als de kans op een Kitten boven een bepaald percentage uitkomt. (Kan je makkelijk uitrekenen, want je weet hoeveel Kittens erin zitten en mag de stapel tellen volgens de regels.) => Ik kan daarvoor gewoon het deck tellen

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