import { handler } from './handler.js';
import express from 'express';
import proxy from 'express-http-proxy';

const app = express();

// add a route that lives separately from the SvelteKit app
app.get('/healthcheck', (req, res) => {
    res.sendStatus(200);
});

app.use("/api", proxy(process.env.BACKEND_URL))
// let SvelteKit handle everything else, including serving prerendered pages and static assets
app.use(handler);

app.listen(3000, () => {
    console.log('listening on port 3000');
});