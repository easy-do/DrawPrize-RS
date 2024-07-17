import {
  Modal,
} from '@arco-design/web-react';
import useLocale from '@/utils/useLocale';
import React from 'react';
import LoginForm from './logionFrom';

function LoginModal(props: {
  visible;
  setVisible;
}) {



  const t = useLocale();


  return (
    <Modal
      style={{width:'20%'}}
      visible={props.visible}
      footer={null}
      onCancel={() => {
        props.setVisible(false);
      }}
      autoFocus={false}
      focusLock={true}
    >

      <LoginForm/>
    </Modal>
  );
}

export default LoginModal;
