//@ts-check
import {FSRS, Card, ScheduledCards, ReviewLog,Rating} from "./index.js"


const fsrs = new FSRS();
const card = new Card();
const scheduled_cards = fsrs.schedule(card, new Date())
let updated_card = scheduled_cards.selectCard(Rating.Easy);
console.log(
updated_card.log().toString()
)
