$(document).ready(() => {
  $.get("/users", (data, _) => {
    $("#title").text(data.title);
    data.users.forEach((user) => {
      console.log(user);
      let txt = `
        <tr>
          <td ${user.id}>${user.name}</td>
          <td>
            <a class="btn btn-warning mx-2" href="../edit/index.html">edit</a>
            <a class="btn btn-danger mx-2" href="../delete/index.html">delete</a>
          </td>
        </tr>
      `;
      $("#users-container").append(txt);
    });
  });
});
