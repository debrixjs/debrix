// @ts-check

const { statSync, readFileSync } = require('fs');
const mime = require('mime');
const { basename } = require('path');

/** @type {any} */
const {
	GITHUB_TOKEN,
	INPUT_REPOSITORY: REPOSITORY,
	INPUT_FILES: FILES,
	INPUT_RELEASE_ID: RELEASE_ID,
} = process.env;

if (!GITHUB_TOKEN) panic('Missing GITHUB_TOKEN!');
if (!REPOSITORY) panic("Missing input 'repository'!");
if (!FILES) panic("Missing input 'files'!");

const [REPOSITORY_OWNER, REPOSITORY_NAME] = REPOSITORY.split('/');

/**
 * @param {TemplateStringsArray} template
 * @param  {...string | number | boolean} args
 */
function url(template, ...args) {
	return String.raw(template, ...args.map((arg) => encodeURIComponent(arg)));
}

/**
 * @param {string} path
 */
function getMimeType(path) {
	return mime.getType(path) || 'application/octet-stream';
}

/**
 * @param  {...any} data
 * @returns {never}
 */
function panic(...data) {
	console.error(...data);
	process.exit(1);
}

/**
 * @param {string} s
 * @returns {string[]}
 */
function parseList(s) {
	/** @type {any} */
	const lines = s.split(/\r?\n/);
	return lines.reduce(
		(acc, line) =>
			acc
				.concat(line.split(','))
				.filter((pat) => pat)
				.map((pat) => pat.trim()),
		[]
	);
}

/**
 * @param {string} path
 */
async function upload(path) {
	const name = basename(path);
	const mime = getMimeType(path);
	const size = statSync(path).size;
	const body = readFileSync(path);
	console.log(`Uploading ${name}...`);
	const endpoint = url`https://uploads.github.com/repos/${REPOSITORY_OWNER}/${REPOSITORY_NAME}/releases/${RELEASE_ID}/assets?name=${name}`;
	const resp = await fetch(endpoint, {
		headers: {
			'content-length': `${size}`,
			'content-type': mime,
			authorization: `token ${GITHUB_TOKEN}`,
		},
		method: 'POST',
		body,
	});
	const json = await resp.json();
	if (resp.status !== 201) {
		throw new Error(
			`Failed to upload release asset ${name}. received status code ${
				resp.status
			}\n${json.message}\n${JSON.stringify(json.errors)}`
		);
	}
}

async function main() {
	const files = parseList(FILES);
	for (const file of files) {
		await upload(file);
	}
}

main();
