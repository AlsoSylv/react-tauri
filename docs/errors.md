### Error handling internally

Rust will return different error keys based on what each error represetns to the current problem. Thsese types of errors include networking errors, data missing errors, and key errors

The follow is a table of errors, codes, and their meanings:

| Errors | Codes | Meaning |
|------------|-----------|-------------|
| Data missing | 101 | Data for combo does not exist |
| Networking Error | 102 | Cannot connect to required server |
| Champ Missing | 103 | Champion does not exist according to Riot |
| Data Dragon Missing | 104 | Data Dragon is missing or not updated |
| CDragon Missing | 105 | All asset servers in use are down* | 
| Role is Invalid | 106 | The role value entered has no data, or is invalid |
| Overview Missing | 201 | UGG Overview JSON is missing |
| Overview Connect Error | 202 | UGG Overview URL changed | 
| Ranking Missing | 203 | UGG Ranking JSON is missing | 
| Ranking Connect Error | 204 | UGG Ranking URL changed | 
| Rate Error | 205 | The Corrosponding rate has no data |
| Matches error | 206 | There are no matches for the selected combo | 
| No Ability Order | 207 | Overview does not contain abilties for combo | 
| Role Missing | 208 | Role JSON is missing |
| Role Connect Error | 209 | Role URL has changed |
| LCU connect error | 401 | No client was found to connect to |
| Cannot delete pages | 402 | No pages were able to be deleted |
| Cannot create pages | 403 | Was not able to create page |
| Cannot get page | 404 | Cannot get current page | 
| Pushed successfully | 405 | Everything worked properly |
*Note that well Community Dragon is not yet used, fallbacks will likely be implemented for it in the future

