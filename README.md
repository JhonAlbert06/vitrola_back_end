# vitrola_back_end

Back-End --- Rust (Server)

This is a Personal Project i make a "jukebox", I used four different programming languages

This is the back end, I used rust for made this, I had no experience with this language but with documentation, videos and some forums I could end.

For the database a used mongoDB 
For the API a used actix-web

This is just one of the four proyects, here are the others: 

----------------------------------------------------------------------
Front-End --- javascript (ReactJS) (Player)

	https://github.com/JhonAlbert06/vitrola_player.git
----------------------------------------------------------------------
Desktop --- Dart (Flutter) (Clien)

	https://github.com/JhonAlbert06/vitrola_desktop.git
----------------------------------------------------------------------
Movil --- Kotlin (jetpack compose) (Clien)

	https://github.com/JhonAlbert06/vitrola_movil.git
----------------------------------------------------------------------

<h3>I plan to implement sockets but I still don't know anything about that so I have to investigate</h3>

<h3>Note:</h3>
 <p> If you want use this proyect, you have to use a file with the name : ".env" ant this is the conten in this file </p>

* Connection string MongoDB
    <p> DATABASE_URL=mongodb://localhost:27017 </p>

* Name Databese
    <p> DATABASE_NAME=playlist </p>

* Tables names in the databases
    <p> USER_COLLECTION_SONGS=songs </p>
    <p> USER_COLLECTION_PLAYLIST=playlist </p>

* IP for locallhost Or in this case IP and Port of the pc
    <p> SERVER_URL=0.0.0.0:8000

This are all the dependencies: 
<ul>
 <li>mongodb = "0.9.2"</>
 <li>actix-web = "2"</>
 <li>actix-files = "0.2.2"</>
 <li>actix-rt = "1"</>
 <li>bson = "0.14.0"</>
 <li>serde = "1.0.103"</>
 <li>futures = "0.3.4"</>
 <li>env_logger = "0.7"</>
 <li>dotenv = "0.15.0"</>
 <li>actix-cors = "0.2.0"</>
 <li>serde_json = "1.0"</>
 <li>pretty_assertions = "1.1.1"</>
</ul>









