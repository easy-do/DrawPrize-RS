import {
  Form,
  FormInstance,
  Input,
  Modal,
  Notification,
} from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { GlobalContext } from '@/context';
import { useContext, useRef } from 'react';
import React from 'react';
import FormItem from '@arco-design/web-react/es/Form/form-item';
import { resetPassword } from '@/api/user';

function ResetPasswordPage(props: {
  id: number;
  visible;
  setVisible;
  callback;
}) {
  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const [loading, setLoading] = React.useState(false);

  const t = useLocale(locale);

  //提交修改
  const handleSubmit = () => {
    setLoading(true);
    formRef.current.validate().then((values) => {
      resetPassword({
        user_id: props.id,
        password: values.password,
      }).then((res) => {
        const { success, message } = res.data;
        if (success) {
          Notification.success({ content: message, duration: 1000 });
          props.setVisible(false);
          props.callback();
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
          label={t['searchTable.columns.password']}
          field={'password'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
      </Form>
    </Modal>
  );
}

export default ResetPasswordPage;
