import {
  Form,
  Input,
  Checkbox,
  Link,
  Button,
  Space,
  Notification,
  Popover,
} from '@arco-design/web-react';
import { FormInstance } from '@arco-design/web-react/es/Form';
import { IconLock, IconUser } from '@arco-design/web-react/icon';
import React, { useEffect, useRef, useState } from 'react';
import useStorage from '@/utils/useStorage';
import useLocale from '@/utils/useLocale';
import { getCaptchaV2, loginRequst, registerRequest } from '@/api/auth';
import locale from './locale';
import styles from './style/index.module.less';

export default function LoginForm() {
  const formRef = useRef<FormInstance>();
  const [errorMessage, setErrorMessage] = useState('');
  const [loading, setLoading] = useState(false);
  const [loginParams, setLoginParams, removeLoginParams] =
    useStorage('loginParams');

  const t = useLocale(locale);

  const [rememberPassword, setRememberPassword] = useState(!!loginParams);

  function afterLoginSuccess(params, data) {
    // 记住密码
    if (rememberPassword) {
      setLoginParams(JSON.stringify(params));
    } else {
      removeLoginParams();
    }
    // 记录登录状态
    localStorage.setItem('Authorization', data);
    // 跳转首页
    // window.location.pathname = '/home'; // 开发环境
    window.location.pathname = '/static/home.html';//生产环境 需要配置成静态文件，否则会报404错误
  }

  function login(params) {
    setErrorMessage('');
    setLoading(true);
    loginRequst(params)
      .then((res) => {
        const { code, success, message, data } = res.data;
        if (code == 200 && success) {
          afterLoginSuccess(params, data);
        } else {
          setErrorMessage(message || t['login.form.login.errMsg']);
        }
      })
      .finally(() => {
        setLoading(false);
      });
  }

  function onLoginSubmitClick() {
    formRef.current.validate().then((values) => {
      login(values);
    });
  }

  const [openRegisterModal, setOpenRegisterModal] = useState(false);

  function openRegister() {
    setOpenRegisterModal(true);
  }

  //验证码
  const [captchaData, setCaptchaData] = useState({
    captcha_base64: '',
    captcha_key: ''
  });

  const getCaptchaData = () => {
    getCaptchaV2().then(res=>{
      const { data } = res.data;
      setCaptchaData(data);
    })
  };

  function onRegisterSubmitClick() {
    formRef.current.validate().then((values) => {
      values.captcha_key = captchaData.captcha_key;
      registerRequest(values).then((res) => {
        const { code, success, message} = res.data;
        if (code == 200 && success) {
          Notification.success({ content: message, duration: 3000 })
          setOpenRegisterModal(false);
          formRef.current.setFieldsValue({
            'username': values.user_name,
            'password': values.password
          });
        } 
      })
      .finally(() => {
        setLoading(false);
      });
    });
  }

  // 读取 localStorage，设置初始值
  useEffect(() => {
    const rememberPassword = !!loginParams;
    setRememberPassword(rememberPassword);
    if (formRef.current && rememberPassword) {
      const parseParams = JSON.parse(loginParams);
      formRef.current.setFieldsValue(parseParams);
    }
    if(openRegisterModal){
      getCaptchaData();
    }
  }, [loginParams, openRegisterModal]);

  return (
    <div className={styles['login-form-wrapper']}>
      <div className={styles['login-form-title']}>{ openRegisterModal? t['login.form.register'] : t['login.form.title']}</div>
      <div className={styles['login-form-error-msg']}>{errorMessage}</div>
      {!openRegisterModal ? (
        <Form
          className={styles['login-form']}
          layout="vertical"
          ref={formRef}
        >
          <Form.Item
            field="username"
            rules={[
              { required: true, message: t['login.form.userName.errMsg'] },
            ]}
          >
            <Input
              prefix={<IconUser />}
              placeholder={t['login.form.userName.placeholder']}
              onPressEnter={onLoginSubmitClick}
            />
          </Form.Item>
          <Form.Item
            field="password"
            rules={[
              { required: true, message: t['login.form.password.errMsg'] },
            ]}
          >
            <Input.Password
              prefix={<IconLock />}
              placeholder={t['login.form.password.placeholder']}
              onPressEnter={onLoginSubmitClick}
            />
          </Form.Item>
          <Space size={16} direction="vertical">
            <div className={styles['login-form-password-actions']}>
              <Checkbox
                checked={rememberPassword}
                onChange={setRememberPassword}
              >
                {t['login.form.rememberPassword']}
              </Checkbox>
            </div>
            <Button
              type="primary"
              long
              onClick={onLoginSubmitClick}
              loading={loading}
            >
              {t['login.form.login']}
            </Button>
            <Button
              type="text"
              long
              className={styles['login-form-register-btn']}
              onClick={() => openRegister()}
            >
              {t['login.form.register']}
            </Button>
          </Space>
        </Form>
      ) : (
        <Form
          className={styles['login-form']}
          layout="vertical"
          ref={formRef}
        >
          <Form.Item
          label={t['register.form.user_name']}
            field="user_name"
            rules={[
              { required: true, message: t['register.form.user_name.errMsg'] },
            ]}
          >
            <Input
              prefix={<IconUser />}
              placeholder={t['login.form.placeholder']}
              onPressEnter={onRegisterSubmitClick}
            />
          </Form.Item>
          <Form.Item
          label={t['register.form.nick_name']}
            field="nick_name"
            rules={[
              { required: true, message: t['register.form.nick_name.errMsg'] },
            ]}
          >
            <Input
              prefix={<IconUser />}
              placeholder={t['login.form.placeholder']}
              onPressEnter={onRegisterSubmitClick}
            />
          </Form.Item>
          <Form.Item
            label={t['register.form.email']}
            field="email"
            rules={[
              { required: true, message: t['register.form.email.errMsg'] },
            ]}
          >
            <Input
              prefix={<IconUser />}
              placeholder={t['login.form.placeholder']}
              onPressEnter={onRegisterSubmitClick}
            />
          </Form.Item>
          <Form.Item
            label={t['register.form.password']}
            field="password"
            rules={[
              { required: true, message: t['register.form.password.errMsg'] },
            ]}
          >
            <Input.Password
              prefix={<IconLock />}
              placeholder={t['login.form.placeholder']}
              onPressEnter={onRegisterSubmitClick}
            />
          </Form.Item>
          <Popover
        trigger='click'
        content={
          <img
          style={{
            width: '100%',
            height: '100px',
            verticalAlign: 'middle',
            padding: '0px 0px 0px 0px',
          }}
          src={'data:image/png;base64,'+ captchaData?.captcha_base64}
          onClick={getCaptchaData}
        />
        }
      >
          <Form.Item
            label={t['register.form.captcha']}
            field="captcha"
            rules={[
              { required: true, message: t['register.form.captcha.errMsg'] },
            ]}
          >

                  <Input
              placeholder={t['login.form.placeholder']}
              onPressEnter={onRegisterSubmitClick}
            />

          </Form.Item>
      </Popover>

          <Space size={16} direction="vertical">
            <Button
              type="primary"
              long
              onClick={onRegisterSubmitClick}
              loading={loading}
            >
              {t['register.form.submit']}
            </Button>
            <Button
              type="text"
              long
              onClick={()=>setOpenRegisterModal(false)}
              loading={loading}
            >
              {t['register.form.backlogin']}
            </Button>
          </Space>
        </Form>
      )}
    </div>
  );
}
