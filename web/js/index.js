// 显示登录表单，隐藏注册表单
function toggleToLogin() {
  document.getElementById('register-form').classList.add('hidden');
  document.getElementById('login-form').classList.remove('hidden');
  document.getElementById('register-message').innerHTML = '';
}

// 显示注册表单，隐藏登录表单
function toggleToRegister() {
  document.getElementById('login-form').classList.add('hidden');
  document.getElementById('register-form').classList.remove('hidden');
  document.getElementById('login-message').innerHTML = '';
}

// 显示消息
function showMessage(containerId, message, isError) {
  const container = document.getElementById(containerId);
  const messageClass = isError ? 'error-message' : 'success-message';
  container.innerHTML = `<div class="message ${messageClass}">${message}</div>`;
}

// 获取URL参数
function getUrlParameter(name) {
  name = name.replace(/[\[\]]/g, '\\$&');
  var regex = new RegExp('[?&]' + name + '(=([^&#]*)|&|#|$)');
  var results = regex.exec(window.location.href);
  if (!results) return null;
  if (!results[2]) return '';
  return decodeURIComponent(results[2].replace(/\+/g, ' '));
}

// 登录处理
function handleLogin() {
  const username = document.getElementById('login-username').value;     // 实际登录请求逻辑
  const password = document.getElementById('login-password').value;
  const loginData = {
    username: username,
    password: password,
    remember: false
  };

  fetch('/api/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(loginData)
  }).then(response => {
    return response.json();
  }).then(data => {
    console.log('请求成功:', data);
    // 登录成功，跳转或其他逻辑
    if (data.success) {
      // 从地址栏获取 callbackUrl 参数
      const callbackUrl = getUrlParameter('callbackUrl');

      // 检查是否有数据和 token
      const hasToken = data.data && data.data.token;

      // 如果有 callbackUrl 且返回了 token，则跳转到 callbackUrl 并携带 token
      if (callbackUrl && hasToken) {
        try {
          // 构造回调 URL，将 token 作为查询参数传递
          const callbackWithToken = new URL(callbackUrl);
          callbackWithToken.searchParams.append('token', data.data.token);

          // 跳转到回调 URL
          window.location.href = callbackWithToken.href;
          return; // 停止后续执行
        } catch (error) {
          console.error('构造回调 URL 失败:', error);
          // 如果构造 URL 失败，显示错误消息
          showMessage('login-message', '回调 URL 无效', true);
          return; // 停止后续执行
        }
      } else {
        // 没有 callbackUrl 或没有 token，显示登录成功消息
        showMessage('login-message', '登录成功！', false);
      }
    } else {
      showMessage('login-message', data.message || '用户名或密码错误', true);
    }

  }).catch(error => {
    console.error('登录失败:', error);
    showMessage('login-message', '登录失败', true);
  });

}

// 注册处理
function handleRegister() {
  const username = document.getElementById('register-username').value.trim();
  const email = document.getElementById('register-email').value.trim();
  const password = document.getElementById('register-password').value;
  const confirmPassword = document.getElementById('register-confirm-password').value;

  // 验证输入
  if (!username || !email || !password || !confirmPassword) {
    showMessage('register-message', '请填写所有字段', true);
    return;
  }

  if (password !== confirmPassword) {
    showMessage('register-message', '两次密码输入不一致', true);
    return;
  }

  if (password.length < 6) {
    showMessage('register-message', '密码长度不能少于6位', true);
    return;
  }

  const registerData = {
    username: username,
    email: email,
    password: password,
    confirm_password: confirmPassword
  };

  fetch('/api/auth/register', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(registerData)
  }).then(response => {
    return response.json();
  }).then(data => {
    console.log('请求成功:', data);
    if (!data.success) {
      showMessage('register-message', data.message || '注册失败', true);
      return;
    }

    // 从地址栏获取 callbackUrl 参数
    const callbackUrl = getUrlParameter('callbackUrl');

    // 检查是否有数据和 token
    const hasToken = data.data && data.data.token;

    // 如果有 callbackUrl 且返回了 token，则跳转到 callbackUrl 并携带 token
    if (callbackUrl && hasToken) {
      try {
        // 构造回调 URL，将 token 作为查询参数传递
        const callbackWithToken = new URL(callbackUrl);
        callbackWithToken.searchParams.append('token', data.data.token);

        // 跳转到回调 URL
        window.location.href = callbackWithToken.href;
        return; // 停止后续执行
      } catch (error) {
        console.error('构造回调 URL 失败:', error);
        // 如果构造 URL 失败，显示错误消息
        showMessage('register-message', '回调 URL 无效', true);
        return; // 停止后续执行
      }
    } else {
      // 没有 callbackUrl 或没有 token，显示注册成功消息并切换到登录表单
      showMessage('register-message', data.message || '注册成功', false);

      // 3秒后自动切换到登录表单
      setTimeout(() => {
        toggleToLogin();
        // 自动填充用户名
        document.getElementById('login-username').value = username;
        // 清空密码框
        document.getElementById('login-password').value = '';
      }, 3000);
    }

  }).catch(error => {
    console.error('注册失败:', error);
    showMessage('register-message', '注册失败', true);
  });

}
