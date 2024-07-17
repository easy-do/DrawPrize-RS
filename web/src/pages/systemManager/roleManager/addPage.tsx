import React, { useContext, useRef } from 'react';
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
import { addRole } from '@/api/role';
import FormItem from '@arco-design/web-react/es/Form/form-item';

function AddPage(props: { visible; setVisible; callback: () => void }) {
  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const t = useLocale(locale);

  const [loading, setLoading] = React.useState(false);

  const handleSubmit = () => {
    formRef.current.validate().then((values) => {
      setLoading(true);
      addRole(values).then((res) => {
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
      title={t['searchTable.operations.add']}
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
          label={t['searchTable.columns.role_name']}
          field={'role_name'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.role_code']}
          field={'role_code'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
      </Form>
    </Modal>
  );
}

export default AddPage;
