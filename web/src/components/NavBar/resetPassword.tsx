import {
  Form,
  FormInstance,
  Input,
  Modal,
  Notification,
} from '@arco-design/web-react';
import useLocale from '@/utils/useLocale';
import { GlobalContext } from '@/context';
import { useContext, useRef } from 'react';
import React from 'react';
import FormItem from '@arco-design/web-react/es/Form/form-item';
import { resetPassword } from '@/api/auth';

function ResetPasswordPage(props: {
  visible;
  setVisible;
  logout
}) {
  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const [loading, setLoading] = React.useState(false);

  const t = useLocale();

  //提交修改
  const handleSubmit = () => {
    setLoading(true);
    formRef.current.validate().then((values) => {
      resetPassword({
        password: values.password,
      }).then((res) => {
        const { success, message, data } = res.data;
        if (success && data) {
          Notification.success({ content: message, duration: 1000 });
          props.setVisible(false);
          props.logout();
        }
      }).finally(()=>{
        setLoading(false);
      });
    });
  };

  return (
    <Modal
      title={t['searchTable.update.title']}
      visible={props.visible}
      onOk={() => {
        handleSubmit();
      }}
      onCancel={() => {
        props.setVisible(false);
      }}
      autoFocus={false}
      focusLock={true}
      confirmLoading={loading}
    >
      <Form
        ref={formRef}
        style={{ width: '95%', marginTop: '6px' }}
        labelCol={{ span: lang === 'en-US' ? 7 : 6 }}
        wrapperCol={{ span: lang === 'en-US' ? 17 : 18 }}
      >
        <FormItem
          required
          label={t['searchTable.columns.id']}
          field={'id'}
          hidden
        >
          <Input placeholder="" />
        </FormItem>
        <FormItem
          required
          label={t['navbar.password']}
          field={'password'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
      </Form>
    </Modal>
  );
}

export default ResetPasswordPage;
