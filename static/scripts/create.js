const URL = "http://localhost:8080";

$(document).ready(() => {
  $("#user-form").submit((event) => {
    event.preventDefault();
    let fname = $("#user-name").val().trim();
    if (fname === "") {
      $("#form-error").text("no name provided");
      return;
    }
    $("#form-error").text("");
    $.ajax({
      url: URL + "/users",
      method: "POST",
      data: JSON.stringify({ name: fname }),
      contentType: "application/json",
    })
      .done((_) => {
        window.location.href = URL + "/static/home.html";
      })
      .fail((_, status, message) =>
        console.log(`status: ${status}\nmessage: ${message}`),
      );
  });
});
