const URL = "http://localhost:8080";

$(document).ready(() => {
  $.get(URL + "/users", function (data, status) {
    console.info(`status: ${status}`);
    if (status != "success") {
      console.log("something went wrong...");
      return;
    }
    $("#title").text(data.title);
    data.users.forEach(function (user) {
      let txt = `
        <tr>
          <td>${user.name}</td>
          <td>
            <button class="btn btn-warning mx-2 action" id="edit-${user.id}" href="#">edit</button>
            <button class="btn btn-danger mx-2 action" id="delete-${user.id}" href="#">delete</button>
          </td>
        </tr>
      `;
      $("#users-container").append(txt);
    });
    $(".action").click(function () {
      let btnId = $(this).closest("button").attr("id").split("-");
      let action = btnId[0];
      let id = btnId[1];
      switch (action) {
        case "edit":
          console.log("this is editing");
          break;
        case "delete":
          console.log("this is deleting lole");
          break;
        default:
          console.log("error... guh??");
          break;
      }
      console.log(action, id);
    });
  });
});
