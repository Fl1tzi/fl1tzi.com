var cal = null;
var ele = document.getElementById('calendar');			
var modal = document.getElementById("event-modal");
var modal_text = document.getElementById("modal-text");
var span = document.getElementsByClassName("close")[0]
var opts = {
	events: [
		{date: new Date(2022, 8-1, 24), desc:"Test"},
		{date: new Date(2022, 8-1, 24), desc:"Test2"},
		{date: new Date(2022, 8-1, 24), desc:"Test3"},
	],
	abbrMonth: false,
	abbrDay: true,
	onEventClick: open_modal
};
cal = new calendar(ele, opts);

function open_modal(ev) {
	modal.style.display = "block";
	modal_text.innerHTML = ev.desc;
}

span.onclick = function() {
  modal.style.display = "none";
}

window.onclick = function(event) {
  if (event.target == modal) {
    modal.style.display = "none";
  }
}
