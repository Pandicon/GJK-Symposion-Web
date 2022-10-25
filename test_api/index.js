const app = require('express')();
const PORT = 8079;

app.listen(PORT, () => console.log(`API runs on localhost:${PORT}`));

app.get('/streda', (req, res) => {
	res.status(200);
	res.send({
		data: {
			harmonogram: [
				[
					null,
					{
						lecturer: '',
						title: 'LCH',
						for_younger: false,
						id: null
					},
					{
						lecturer: '',
						title: 'Sklep GJK',
						for_younger: false,
						id: '0-2'
					},
					{
						lecturer: '',
						title: 'Strecha GJK',
						for_younger: false,
						id: '0-3'
					}
				],
				[
					{
						lecturer: '',
						title: '9:05 - 11:05',
						for_younger: false,
						id: null
					},
					{
						lecturer: "<script>alert('cheche!');</script>",
						title: "<script>alert('hehe!');</script>",
						for_younger: false,
						id: null,
						row_span: 2
					},
					null,
					null
				],
				[
					{
						lecturer: '',
						title: '11:05 - 11:55',
						for_younger: false,
						id: null
					},
					{
						lecturer: 'prednasejici #1',
						title: 'vysoce narocne tema tykajici se mostu',
						for_younger: false,
						id: '2-2'
					},
					null
				],
				[
					{
						lecturer: '',
						title: '12:01 - 13:02',
						for_younger: false,
						id: null
					},
					{
						lecturer: 'prednasejici #3',
						title: 'odpalování mostů',
						for_younger: false,
						id: '3-1'
					},
					{
						lecturer: 'pan prednasejici #1',
						title: 'symposion web stranky jako most mezi organizatory a ucastniky',
						for_younger: true,
						id: '3-2',
						row_span: 2
					},
					{
						lecturer: 'pani prednasejici #1',
						title: 'rezonance mostu ve vetru',
						for_younger: true,
						id: '3-3',
						row_span: 2
					}
				],
				[
					{
						lecturer: '',
						title: '13:02 - 23:42',
						for_younger: false,
						id: null
					},
					null
				],
				[
					{
						lecturer: '',
						title: '23:42 - 23:57',
						for_younger: false,
						id: null
					},
					{
						lecturer: '',
						title: 'VEČEŘE',
						for_younger: true,
						id: '5-1',
						col_span: 3
					}
				],
				[
					{
						lecturer: '',
						title: '23:59 - 24:00',
						for_younger: false,
						id: null
					},
					null,
					null,
					{
						lecturer: 'prednasejici #2',
						title: 'pozorování hvězd na téma most',
						for_younger: false,
						id: '6-3'
					}
				]
			],
			last_updated: 1666606780
		},
		error: null
	});
});

app.get('/ctvrtek', (req, res) => {
	res.status(500);
	res.send({
		data: null,
		error: 'Nejsou data o harmonogramu'
	});
});

app.get('/patek', (req, res) => {
	res.status(500);
	res.send({
		data: null,
		error: null
	});
});

const anotace = {
	streda: {
		'2-2': {
			data: null,
			error: null
		},
		'3-3': {
			data: {
				info: {
					annotation: 'Krásná anotace',
					lecturer_info: 'Krásný medajlonek'
				},
				last_updated: 1666606780
			},
			error: null
		},
		'6-3': {
			data: null,
			error: 'O této přednášce nejsou žádná data'
		}
	}
};

app.get('/anotace/:day/:id', (req, res) => {
	const { day, id } = req.params;
	if (anotace[day] != null && anotace[day] != undefined) {
		res.send(anotace[day][id] ?? {});
	} else {
		res.send({});
	}
});
