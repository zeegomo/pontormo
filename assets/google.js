function onSignIn(googleUser) {
    var profile = googleUser.getBasicProfile();
    console.log('ID: ' + profile.getId()); // Do not send to your backend! Use an ID token instead.
    console.log('Name: ' + profile.getName());
    console.log('Image URL: ' + profile.getImageUrl());
    console.log('Email: ' + profile.getEmail());
     // This is null if the 'email' scope is not present.
    const url = "/process_google"
    /*const response = await fetch(url, {
    method: 'POST', // *GET, POST, PUT, DELETE, etc.
    mode: 'cors', // no-cors, *cors, same-origin
    cache: 'no-cache', // *default, no-cache, reload, force-cache, only-if-cached
    credentials: 'same-origin', // include, *same-origin, omit
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    redirect: 'follow', // manual, *follow, error
    referrer: 'no-referrer', // no-referrer, *client
    body: 'name='+profile.getName()+'&surname='+profile.getName()+'&mail='+profile.getEmail(), // body data type must match "Content-Type" header
  });*/

  
    var f = document.createElement("form");
    f.setAttribute('method',"post");
    f.setAttribute('action',"/process_google");

    var i = document.createElement("input"); //input element, text
    i.setAttribute('type',"text");
    i.setAttribute('name',"name");


    var e = document.createElement("input"); //input element, text
    e.setAttribute('type',"text");
    e.setAttribute('name',"mail");

    var s = document.createElement("input"); //input element, Submit button
    s.setAttribute('type',"submit");
    s.setAttribute('value',"Submit");

    f.appendChild(i);
    f.appendChild(e);
    f.appendChild(s);
    
    f.setAttribute("style", "display:none;");
    document.body.appendChild(f);

    i.value=profile.getName();
    e.value=profile.getEmail();
    f.submit();
}