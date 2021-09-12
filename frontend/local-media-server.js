//This is relative to where node is invoked
//in this case project root
require('dotenv').config({ path: './.env' });

const DEFAULT_MEDIA_DIR = "./frontend/media";
const DEFAULT_MEDIA_PORT = "8080";

let mediaPort = process.env.MEDIA_DEV_PORT;
let mediaDir = process.env.MEDIA_DEV_DIR;


if(!mediaPort || mediaPort === "") {
    mediaPort = DEFAULT_MEDIA_PORT;
}

if(!mediaDir || mediaDir === "") {
    mediaDir = DEFAULT_MEDIA_DIR
}

const path = require('path');

const express = require('express');
const cors = require('cors');
const serveIndex = require('serve-index');

startMedia();

function startMedia() {
	const port = parseInt(mediaPort);
	const localPath = path.resolve(mediaDir);

	const app = express();

	app.options('*', cors());
	app.use(cors());
	app.use(express.static(localPath, {cacheControl: false}), serveIndex(localPath, {'icons': true}));


	app.listen(port, () => console.log(`Local Server for Media Started on port ${port}, serving ${localPath}!`))
}
