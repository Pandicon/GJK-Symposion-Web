/**
 * Parses the harmonogram data
 *
 * @param {Array<Array<string>>} harmonogram The harmonogram we get from the team
 * @return The harmonogram for the web
 * @customfunction
 */
function harmonogram(harmonogram) {
  let table = cut_table(harmonogram);
  return table;
}

/**
 * Cuts out the relevant data from the table, leaving out anything after an empty column and an empty row
 *
 * @param {Array<Array<string>>} table The table to cut
 * @return The cut table
 * @customfunction
 */
function cut_table(table) {
  let cut_table = [];
  for(const row of table) {
    if(row.filter(element => element.trim() != "").length == 0) break;
    cut_table.push(row);
  }
  for(let i = 0; i < cut_table[0].length; i += 1) {
    let empty = true;
    for(const row of table) {
      if(row[i].trim() != "") {
        empty = false;
        break;
      }
    }
    if(empty) {
      for(const row of table) {
        row.length = i;
      }
      break;
    };
  }
  console.log(cut_table);
  return cut_table;
}