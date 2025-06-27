const URL = "http://localhost:8080";

$(document).ready(() => {
  $.get(URL + "/users", (data, status) => {
    console.info(`status: ${status}`);
    if (status != "success") {
      console.log("something went wrong...");
      return;
    }
    $("#title").text(data.title);
    data.users.forEach((user) => {
      let txt = `
        <tr>
          <td ${user.id}>${user.name}</td>
          <td>
            <button class="btn btn-warning mx-2 action" id="edit-${user.id}" href="#">edit</button>
            <button class="btn btn-danger mx-2 action" id="delete-${user.id}" href="#">delete</button>
          </td>
        </tr>
      `;
      $("#users-container").append(txt);
    });
    // TODO: when clicking an action button, get the id
    $(".action").click(() => {
      console.log($(this));
    });
  });
});
