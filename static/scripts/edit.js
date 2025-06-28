const ID = new URLSearchParams(window.location.search).get("id");
const URL = "http://localhost:8080";

$(document).ready(function () {
  $.get(`${URL}/users/${ID}`, (data, status) => {
    console.info(`status: ${status}`);
    if (status != "success") {
      console.log("something went wrong...");
      return;
    }
    $("#user-name").val(data.user.name);
  });

  $("#user-form").submit((event) => {
    event.preventDefault();
    let fname = $("#user-name").val().trim();
    if (fname === "") {
      $("#form-error").text("no name provided");
      return;
    }
    $("#form-error").text("");
    $.ajax({
      url: `${URL}/users/${ID}`,
      method: "PUT",
      data: JSON.stringify({ name: fname }),
      contentType: "application/json",
    })
      .done((_) => {
        window.location.href = `${URL}/static/home.html`;
      })
      .fail((_, status, message) =>
        console.log(`status: ${status}\nmessage: ${message}`),
      );
  });
});
