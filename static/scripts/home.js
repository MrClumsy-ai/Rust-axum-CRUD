const URL = "http://localhost:8080";

$(document).ready(() => {
  $.get(URL + "/users", (data, status) => {
    console.info(`status: ${status}`);
    if (status != "success") {
      console.log(`Something went wrong: ${status}`);
      return;
    }
    $("#title").text(data.title);
    data.users.forEach((user) => {
      $("#users-container").append(`
        <tr>
          <td>${user.name}</td>
          <td>
            <button class="btn btn-warning mx-2 action" id="edit-${user.id}" href="#">Edit</button>
            <button class="btn btn-danger mx-2 action" id="delete-${user.id}" href="#">Delete</button>
          </td>
        </tr>
      `);
    });
    $(".action").click(function () {
      let btnId = $(this).closest("button").attr("id").split("-");
      let action = btnId[0];
      let id = btnId[1];
      switch (action) {
        case "edit":
          window.location =
            `${URL}/static/edit.html?id=` + encodeURIComponent(id);
          break;
        case "delete":
          window.location =
            `${URL}/static/delete.html?id=` + encodeURIComponent(id);
          break;
        default:
          console.log("error... guh??");
          break;
      }
      console.log(action, id);
    });
  });
});
