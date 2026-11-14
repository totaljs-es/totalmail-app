// The page of the app: the address of the server, once. Then the window shows the server (connect in src-tauri/src/lib.rs).
// The texts are the ones of the Apple app (Localizable.xcstrings), in its eight languages.
var TEXTS = {
	"en": {
		"welcome": "Welcome to TotalMail",
		"enter": "Enter the address of your TotalMail.",
		"continue": "Continue",
		"invalid": "That is not a valid address.",
		"https": "Use https: a password must not travel in the clear across the internet.",
		"nottm": "This address does not answer like a TotalMail.",
		"unreachable": "The server cannot be reached. Check the address and your connection.",
		"down": "TotalMail cannot reach your server right now.",
		"other": "Use another server",
		"retry": "Try again"
	},
	"es": {
		"welcome": "Te damos la bienvenida a TotalMail",
		"enter": "Escribe la dirección de tu TotalMail.",
		"continue": "Continuar",
		"invalid": "Esa dirección no es válida.",
		"https": "Usa https: una contraseña no debe viajar sin cifrar por internet.",
		"nottm": "Esta dirección no responde como un TotalMail.",
		"unreachable": "No se puede contactar con el servidor. Revisa la dirección y tu conexión.",
		"down": "TotalMail no puede contactar con tu servidor ahora mismo.",
		"other": "Usar otro servidor",
		"retry": "Reintentar"
	},
	"sk": {
		"welcome": "Vitajte v TotalMail",
		"enter": "Zadajte adresu vášho TotalMailu.",
		"continue": "Pokračovať",
		"invalid": "Táto adresa nie je platná.",
		"https": "Použite https: heslo nesmie cestovať internetom nešifrované.",
		"nottm": "Táto adresa neodpovedá ako TotalMail.",
		"unreachable": "Server sa nedá kontaktovať. Skontrolujte adresu a pripojenie.",
		"down": "TotalMail sa teraz nemôže spojiť s vaším serverom.",
		"other": "Použiť iný server",
		"retry": "Skúsiť znova"
	},
	"de": {
		"welcome": "Willkommen bei TotalMail",
		"enter": "Geben Sie die Adresse Ihres TotalMail ein.",
		"continue": "Fortfahren",
		"invalid": "Das ist keine gültige Adresse.",
		"https": "Verwenden Sie https: Ein Passwort darf nicht unverschlüsselt durchs Internet reisen.",
		"nottm": "Diese Adresse antwortet nicht wie ein TotalMail.",
		"unreachable": "Der Server ist nicht erreichbar. Prüfen Sie die Adresse und Ihre Verbindung.",
		"down": "TotalMail kann Ihren Server im Moment nicht erreichen.",
		"other": "Anderen Server verwenden",
		"retry": "Erneut versuchen"
	},
	"fr": {
		"welcome": "Bienvenue dans TotalMail",
		"enter": "Saisissez l’adresse de votre TotalMail.",
		"continue": "Continuer",
		"invalid": "Ce n’est pas une adresse valide.",
		"https": "Utilisez https : un mot de passe ne doit pas circuler en clair sur Internet.",
		"nottm": "Cette adresse ne répond pas comme un TotalMail.",
		"unreachable": "Le serveur est injoignable. Vérifiez l’adresse et votre connexion.",
		"down": "TotalMail ne peut pas joindre votre serveur pour le moment.",
		"other": "Utiliser un autre serveur",
		"retry": "Réessayer"
	},
	"it": {
		"welcome": "Benvenuto in TotalMail",
		"enter": "Inserisca l’indirizzo del suo TotalMail.",
		"continue": "Continua",
		"invalid": "Questo non è un indirizzo valido.",
		"https": "Usi https: una password non deve viaggiare in chiaro su Internet.",
		"nottm": "Questo indirizzo non risponde come un TotalMail.",
		"unreachable": "Impossibile raggiungere il server. Controlli l’indirizzo e la connessione.",
		"down": "TotalMail non riesce a raggiungere il suo server in questo momento.",
		"other": "Usa un altro server",
		"retry": "Riprova"
	},
	"pt": {
		"welcome": "Bem-vindo ao TotalMail",
		"enter": "Introduza o endereço do seu TotalMail.",
		"continue": "Continuar",
		"invalid": "Esse não é um endereço válido.",
		"https": "Use https: uma palavra-passe não deve viajar em claro pela internet.",
		"nottm": "Este endereço não responde como um TotalMail.",
		"unreachable": "Não foi possível contactar o servidor. Verifique o endereço e a ligação.",
		"down": "O TotalMail não consegue contactar o seu servidor neste momento.",
		"other": "Usar outro servidor",
		"retry": "Tentar novamente"
	},
	"el": {
		"welcome": "Καλώς ήρθατε στο TotalMail",
		"enter": "Εισαγάγετε τη διεύθυνση του TotalMail σας.",
		"continue": "Συνέχεια",
		"invalid": "Αυτή δεν είναι έγκυρη διεύθυνση.",
		"https": "Χρησιμοποιήστε https: ένας κωδικός δεν πρέπει να ταξιδεύει ακρυπτογράφητος στο διαδίκτυο.",
		"nottm": "Αυτή η διεύθυνση δεν απαντά σαν TotalMail.",
		"unreachable": "Δεν ήταν δυνατή η επικοινωνία με τον διακομιστή. Ελέγξτε τη διεύθυνση και τη σύνδεσή σας.",
		"down": "Το TotalMail δεν μπορεί να επικοινωνήσει με τον διακομιστή σας αυτή τη στιγμή.",
		"other": "Χρήση άλλου διακομιστή",
		"retry": "Δοκιμάστε ξανά"
	}
};

var lang = (navigator.languages || [navigator.language || 'en'])
	.map(function(l) { return String(l).slice(0, 2).toLowerCase(); })
	.find(function(l) { return TEXTS[l]; }) || 'en';
var T = TEXTS[lang];
var invoke = window.__TAURI_INTERNALS__.invoke;
var form = document.getElementById('form'),
	input = document.getElementById('address'),
	button = document.getElementById('continue'),
	error = document.getElementById('error'),
	down = document.getElementById('down');

document.documentElement.lang = lang;
document.querySelectorAll('[data-t]').forEach(function(el) { el.textContent = T[el.dataset.t]; });

function busy(on) {
	button.disabled = on;
	input.disabled = on;
	document.body.classList.toggle('busy', on);
}

async function connect(address, saved) {
	error.textContent = '';
	busy(true);
	try {
		await invoke('connect', { address: address });
	} catch (key) {
		busy(false);
		// A saved server that does not answer right now: try again, or choose another one.
		if (saved && key === 'unreachable') {
			form.hidden = true;
			down.hidden = false;
			return;
		}
		error.textContent = T[key] || T.unreachable;
		input.focus();
	}
}

form.addEventListener('submit', function(e) {
	e.preventDefault();
	connect(input.value, false);
});
document.getElementById('retry').addEventListener('click', function() {
	down.hidden = true;
	form.hidden = false;
	connect(input.value, true);
});
document.getElementById('other').addEventListener('click', async function() {
	await invoke('forget');
});

(async function() {
	var saved = await invoke('saved');
	if (saved)
		input.value = saved.replace(/^https:\/\//, '').replace(/\/$/, '');
	if (saved && !/[?&]change=1/.test(location.search))
		connect(saved, true);
	else
		input.focus();
})();
