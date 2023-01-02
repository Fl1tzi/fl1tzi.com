use proc_macro_markdown::md_to_html;

pub struct Post {
    pub number: usize,
    pub name: &'static str,
    pub prev: &'static str,
    pub desc: &'static str,
}

// the posts which are shown
// do NOT use 404
const LEN: usize = 3;
pub const POSTS: [Post; LEN] = 
    [
    Post {
        number: 4,
        name: "[DE] Wieso Whatsapp nicht zu trauen ist",
        prev: "\"Wenn es kein Produkt gibt, bist du das Produkt\"",
        desc: md_to_html!(
"
Für viele ist Whatsapp immer noch die beliebteste Methode um angehörigen angeblich sichere Nachrichten zu schicken.

Wie schlecht Whatsapp aber eigentlich für den Datenschutz ist, wissen leider davon nur wenige.

## 1. E2EE

Niemand weiß wirklich ob die End-zu-End Verschlüsselung richtig implementiert ist. Der Programmcode von Whatsapp liegt nicht offen und kann deswegen nicht von unabhängigen Personen überprüft werden.

Signal hat zum Beispiel ihren Programmcode inklusive Implementierung auf Seiten des Servers [hier](https://github.com/signalapp) veröffentlicht.         

## 2. for-profit

Meta ist ein for-profit Unternehmen, möchte also Profit machen mit ihren Diensten und hat dabei in der Vergangenheit schon einige Male dies demonstriert, wie man in Sachen wie [Cambridge Analytica](https://de.wikipedia.org/wiki/Cambridge_Analytica) gesehen hat.
Dort wurde versucht Facebook Nutzer durch Analyse der Nutzerdaten so zu beeinflussen, dass deren Wahlverhalten verändert werden würde.
Das soll übrigens 2016 maßgeblich zu Trumps Wahlsieg geholfen haben. Also zeigten die 5,9 Millionen Dollar von der Trump-Kampagne¹ anscheinend Wirkung. Das zeigt also, dass Facebook für genug Geld auf den Datenschutz scheißt.

Ürbigens wurde Meta (damals Facebook) im Jahr 2016 dafür angezeigt private Nachrichten für Werbezwecke (Profit) verwendet zu haben² ³...

- ¹ https://www.derstandard.de/story/2000066710266/trump-kampagne-cambridge-analytica-hatte-kontakt-mit-wikileaks
- ² https://www.slideshare.net/evilhackerz/facebook-lawsuit
- ³ https://www.theverge.com/2016/5/19/11712804/facebook-private-message-scanning-privacy-lawsuit

## 3. Datenschutzerklärung (Vergleich mit Signal)

- [Datenschutzerklärung Whatsapp](https://www.whatsapp.com/legal/privacy-policy-eea)
- [Datenschutzerklärung Signal](https://signal.org/legal/)

---

### Whatsapp

#### Telefonnummer, Profilstatus, Geburtsdatum, Anzeigennamen, Profilbild:
            
<p style=\"color: red;\">(unverschlüsselt)</p>

Werden unter keinen besonderen Bedingungen gespeichert.

#### Metadaten:

<p style=\"color: red;\">(unverschlüsselt)</p>

Werden unter keinen besonderen Bedingungen gespeichert und in hoher Menge.

#### Nachrichten:

<p style=\"color: green;\">(verschlüsselt)</p>
            
Text: Whatsapp versendet diese nach ihrer Verschlüsselung und speichert diese wohl nur kurz und falls der Nutzer nicht erreichbar ist bis zu 30 Tage. Sie können diese angeblich nicht sehen.

Medien: Whatsapp speichert diese bis zu 30 Tage und kann diese angeblich nicht sehen.

#### Kontakte:

<p style=\"color: yellow;\">Undurchsichtiges Hashing der Kontakte und tägliche Überprüfung</p>

Auch wenn Whatsapp hier die Nutzer schützen zu scheint, sieht es so aus als würden sie die Daten von Kontakten über deren Server verarbeiten.

Da man diese Informationen nicht überprüfen kann, kann man nicht verifizieren, dass nicht Kontakte, welche nicht bei Whatsapp registriert sind, nicht verarbeitet werden.

[siehe auch hier](https://faq.whatsapp.com/1191526044909364/?locale=en_US)

#### Externe:

Whatsapp scheint viele externe Dienste zu haben, welche leider nicht sehr durchsichtig in der Datenschutzerklärung dargestellt werden...

- <p style=\"color: grey;\">Behörden (benötigt)</p>
- <p style=\"color: red;\">andere Meta Anbieter</p>
- <p style=\"color: red;\">\"integrierte Services\"</p>

#### Ein großes Problem was teilweise missachtet wird

Metadaten werden immer wichtiger. Es sind diese, welche Whatsapp nicht genau mit einer schönen Ende-zu-Ende-Verschlüsselung thematisiert.
Wir reden hier also von Daten, welche dazu benutzen werden können um soziale Verbindungen zu profilieren und zudem dein Verhalten.

Dabei geht es um deine Telefonnummer, Profilstatus, Geburtsdatum, Anzeigenname, Profilbild, Kontaktbuch, mit wem du kommunizierst und Daten von deinem Telefon, welche dich über Whatsapp hinaus identifizieren.

Zudem können dazu auch Sachen wie länge von Anrufen zu Personen, Kommunikationslänge mit Personen (Anzahl von Nachrichten) oder auch wie lange du auf eine Nachricht wartest und so weiter verwendet werden...

Meta kann alleine mit diesen Informationen wertvolle Daten über dich und über die Personen mit denen du kommunizierst sammeln.

Auch hier wieder: Wir kennen die genauen technischen Implementierungen nicht und können keine genauen Aussagen darüber machen und können den Versprechen deswegen nicht trauen.

---

### Signal

Als erstes: Diese Datenschutzerklärung ist viel kürzer und übersichtlicher für den Nutzer.

#### Telefonnummer, Accountinformationen:

<p style=\"color: green;\">(verschlüsselt)</p>

Signal nutzt eine Ende-zu-Ende-Verschlüsselung für diese Daten.

#### Metadaten:

<p style=\"color: green;\">Werden teilweise verschlüsselt und nur in geringer benötigter Menge gespeichert.</p>

Wie gesagt, der Programmcode ist offen, jeder kann genau nachgucken was gespeichert wird.

Es lohnt sich trotzdem zu bedenken, dass Signal ihre Dienste bei Tech-Giganten hosten und diese die IP eines Nutzers herausfinden könnten auch wenn dies natürlich keine Daten wie Nachrichten oder sonstiges betrifft.
Es sollte sich von selbst klären, dass dies immer noch besser ist als Whatsapp.
Der \"kuketz-blog\" hat dies [hier](https://www.kuketz-blog.de/signal-jegliche-kommunikation-erfolgt-ueber-tech-giganten-wie-amazon-microsoft-google-und-cloudflare/) sehr gut thematisiert.

#### Nachrichten:

<p style=\"color: green;\">(verschlüsselt)</p>

Signal speichert Nachrichten temporär falls der Nutzer nicht erreichbar ist.

#### Kontakte:

<p style=\"color: green;\">keine persönlichen Daten</p>

Dieses System basiert auf dem [Hashing](https://de.wikipedia.org/wiki/Hash) von einer Telefonnumer, welche anschließend in einer [Software Guard Extension](https://en.wikipedia.org/wiki/Software_Guard_Extensions) abgeglichen wird.

#### Externe:

Signal macht es uns sehr leicht diese einzusehen:

- <p style=\"color: grey;\">Behörden (benötigt)</p>
            
über welche Signal sehr transparent ist: [siehe hier](https://signal.org/bigbrother/)

---

## Offenheit ist der Schlüssel zu einer sicheren Kommunikation

Wie oben schon angesprochen ist der Programmcode von Signal veröffentlicht und ist somit vollständig durchsichtig über die Verwendung der Daten. Zwar ist Signal immer noch zentralisiert (nicht wie z.b. [Matrix](https://matrix.org/)) und benötigt immer noch eine Telefonnummer, aber ist wesentlich besser als Whatsapp mit ihren undurchlässigen Informationen, wobei man vom schlimmsten ausgehen muss.

> Signal will einen offenen Dienst anbieten. Meta will verdienen.

Übrigens, für jeden der nochmal andere Messenger vergleichen möchte, gibt es eine schöne [Messenger-Matrix](https://www.messenger-matrix.de/messenger-matrix.html) von Kuketz, welcher Daten ich zustimme (Stand: 2.1.2023).

Du findest in diesem Post ist etwas falsch thematisiert oder du möchtest etwas klären, dann kontaktiere mich bitte über meine E-Mail oder über Matrix. Ich bin offen für Kritik.
#
"
        )
    },
    Post {
        number: 3,
        name: "[DE] Chatkontrolle stoppen!",
        prev: "Eine EU Verordnung gegen die Privatsphäre",
        desc: "
            <p>
            Die EU möchte mithilfe einer digitalen Kommunikationskontrolle die Chats von Benutzern umfassend überwachen.
            </p>
            <p>
            Sämtliche Nachrichten in sozialen Medien sollen hiermit kontrolliert werden und die Ende-zu-Ende Verschlüsselung umgangen werden, sodass Daten gelesen und ausgewertet werden können.
            </p>
            <p>
            Eine gute Übersicht gibts im Artikel vom CCC: <a href=\"https://www.ccc.de/de/updates/2022/eu-kommission-will-alle-chatnachrichten-durchleuchten\">https://www.ccc.de/de/updates/2022/eu-kommission-will-alle-chatnachrichten-durchleuchten</a>
            "
        },
    Post {
        number: 1,
        name: "WASM on this site",
        prev: "This site now runs WebAssembly!",
        desc: md_to_html!(
r"
Every post is rendered in [WebAssembly](https://webassembly.org/). The full source is available at [Fl1tzi/fl1tzi.com](https://github.com/Fl1tzi/fl1tzi.com/).
"
        )
    }
    ];

