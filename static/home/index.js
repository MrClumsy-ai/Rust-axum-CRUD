$(document).ready(() => {
  $.get("/", (data, _) => {
    let msg = data.message;
    console.log(msg);
    $("#title").text(msg);
  });

  $.get("/users", (data, _) => {
    data.users.forEach((user) => {
      console.log(user);
      let txt = `
        <tr>
          <td ${user.id}>${user.name}</td>
          <td>
            <a class="btn btn-warning" href="../edit/index.html?id=${user.id}">edit</a>
            <a class="btn btn-danger" href="../delete/index.html?id=${user.id}">delete</a>
          </td>
        </tr>
      `;
      $("#users-container").append(txt);
    });
  });
});
