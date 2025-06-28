const ID = new URLSearchParams(window.location.search).get("id");
const URL = "http://localhost:8080";

$(document).ready(function () {
  $.get(`${URL}/users/${ID}`, (data, status) => {
    console.info(`status: ${status}`);
    if (status != "success") {
      console.log("something went wrong...");
      return;
    }
    $("#username-container").text(data.user.name);
  });

  $("#delete-btn").click(() => {
    $.ajax({
      url: `${URL}/users/${ID}`,
      method: "DELETE",
    })
      .done((_) => {
        window.location.href = `${URL}/static/home.html`;
      })
      .fail((_, status, message) =>
        console.log(`status: ${status}\nmessage: ${message}`),
      );
  });
});
