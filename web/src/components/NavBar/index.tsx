import React, { useContext, useEffect, useState } from 'react';
import {
  Tooltip,
  Input,
  Avatar,
  Select,
  Dropdown,
  Menu,
  Divider,
  Message,
  Button,
} from '@arco-design/web-react';
import {
  IconLanguage,
  IconNotification,
  IconSunFill,
  IconMoonFill,
  IconSettings,
  IconPoweroff,
  IconLoading,
  IconUserGroup,
  IconUser,
} from '@arco-design/web-react/icon';
import { useSelector } from 'react-redux';
import { GlobalState } from '@/store';
import { GlobalContext } from '@/context';
import useLocale from '@/utils/useLocale';
import MessageBox from '@/components/MessageBox';
import IconButton from './IconButton';
import Settings from '../Settings';
import styles from './style/index.module.less';
import defaultLocale from '@/locale';
import { logoutRequest } from '@/api/auth';
import Router from 'next/router';
import ResetPasswordPage from './resetPassword';
import LoginModal from '../LoginModal';

function Navbar({ show }: { show: boolean }) {
  const t = useLocale();
  const { userInfo, userLoading } = useSelector((state: GlobalState) => state);


  const { setLang, lang, theme, setTheme } = useContext(GlobalContext);

  useEffect(() => {
    
    if (!localStorage.getItem('Authorization')) {
      setOpenLoginModal(true);
    }
  }, [JSON.stringify(userInfo)]);

  function logout() {
    logoutRequest().then((res) => {
      const { success } = res.data;
      if (success) {
        localStorage.removeItem('Authorization')
        window.location.pathname = 'home';
      }
    });
  }

  function onMenuItemClick(key) {
    if (key === 'logout') {
      logout();
    }
    if (key === 'setting') {
      Router.push('/user/setting','',{})
    }
    if (key === 'reset_password') {
       setIsSetpass(true)
    }
    if (key === 'login') {
      setOpenLoginModal(true)
    }
  }

  const [isSetpass, setIsSetpass] = useState(false);
  const [openLoginModal, setOpenLoginModal] = useState(false);

  if (!show) {
    return (
      <div className={styles['fixed-settings']}>
        <Settings
          trigger={
            <Button icon={<IconSettings />} type="primary" size="large" />
          }
        />
      </div>
    );
  }


  const droplist = (
    <Menu onClickMenuItem={onMenuItemClick}>
      {!userInfo.uid ? (
        <Menu.Item key="login">
          <IconUserGroup className={styles['dropdown-icon']} />
          {t['navbar.login']}
        </Menu.Item>
      ) : null}
      {userInfo.uid ? (
        <Menu.Item key="nick_name">
          <IconUserGroup className={styles['dropdown-icon']} />
          {userInfo.nick_name}
        </Menu.Item>
      ) : null}
      {userInfo.uid ? (
        <Menu.Item key="reset_password">
          <IconPoweroff className={styles['dropdown-icon']} />
          {t['navbar.reset_password']}
        </Menu.Item>
      ) : null}
      <Divider style={{ margin: '4px 0' }} />
      {userInfo.uid? (
        <Menu.Item key="logout">
          <IconPoweroff className={styles['dropdown-icon']} />
          {t['navbar.logout']}
        </Menu.Item>
      ) : null}
    </Menu>
  );

  return (
    <div className={styles.navbar}>
      <div className={styles.left}>
        <div className={styles.logo}>
          <div className={styles['logo-name']}>Draw-RS</div>
        </div>
      </div>
      <ul className={styles.right}>
        <li>
          <Input.Search
            className={styles.round}
            placeholder={t['navbar.search.placeholder']}
          />
        </li>
        <li>
          <Select
            triggerElement={<IconButton icon={<IconLanguage />} />}
            options={[
              { label: '中文', value: 'zh-CN' },
              { label: 'English', value: 'en-US' },
            ]}
            value={lang}
            triggerProps={{
              autoAlignPopupWidth: false,
              autoAlignPopupMinWidth: true,
              position: 'br',
            }}
            trigger="hover"
            onChange={(value) => {
              setLang(value);
              const nextLang = defaultLocale[value];
              Message.info(`${nextLang['message.lang.tips']}${value}`);
            }}
          />
        </li>
        <li>
          <MessageBox>
            <IconButton icon={<IconNotification />} />
          </MessageBox>
        </li>
        <li>
          <Tooltip
            content={
              theme === 'light'
                ? t['settings.navbar.theme.toDark']
                : t['settings.navbar.theme.toLight']
            }
          >
            <IconButton
              icon={theme !== 'dark' ? <IconMoonFill /> : <IconSunFill />}
              onClick={() => setTheme(theme === 'light' ? 'dark' : 'light')}
            />
          </Tooltip>
        </li>
        <Settings />
        {userInfo && (
          <li>
            <Dropdown droplist={droplist} position="br" disabled={userLoading}>
              <Avatar size={32} style={{ cursor: 'pointer', backgroundColor: '#3370ff'}}>
                {userLoading ? (
                  <IconLoading />
                ) : (
                  // <img alt="avatar" src={'https://img2.baidu.com/it/u=2539468734,376787382&fm=253&fmt=auto&app=138&f=JPEG?w=500&h=375'} />
                  <IconUser />
                )}
              </Avatar>
            </Dropdown>
          </li>
        )}
      </ul>
      <ResetPasswordPage visible={isSetpass} setVisible={setIsSetpass} logout={logout} />
      <LoginModal visible={openLoginModal} setVisible={setOpenLoginModal}/>
    </div>
  );
}

export default Navbar;
