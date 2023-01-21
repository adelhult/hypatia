- [ ] Omvandla alla update uttryck i resolve-fasen
- [ ] Kanske vill ta hänsyn till enheten i "10 m" också
- [x] Faktiskt se till att man inte har något lokalt scope utan använder global om man är längst ut
- [ ] Skriv om resolvern så att den inte skapar noder utan bara uppdaterar från Scope::Unresolved till Scope::Local(...) eller Scope::Global

Om man kör cargo t så pajar några grejer kopplat till update uttryck. Det som ska göras är att skriva om dem så att de använder sig av Scope i resolve-fasen också och sen kan det rättas till i eval.