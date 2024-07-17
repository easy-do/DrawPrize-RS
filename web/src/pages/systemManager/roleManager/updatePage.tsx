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
import { useContext, useEffect, useRef } from 'react';
import React from 'react';
import FormItem from '@arco-design/web-react/es/Form/form-item';
import { getRoleInfo, updateRole } from '@/api/role';

function UpdatePage(props: { id: number; visible; setVisible; callback }) {
  const TextArea = Input.TextArea;

  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const [loading, setLoading] = React.useState(false);

  //加载数据
  function fetchData() {
    if (props.id !== undefined && props.visible) {
      setLoading(true);
      getRoleInfo(props.id).then((res) => {
        const { success, data } = res.data;
        if (success) {
          formRef.current.setFieldsValue(data);
        }
        setLoading(false);
      }).finally(()=>{
        setLoading(false);
      });
    }
  }

  useEffect(() => {
    fetchData();
  }, [props.id, props.visible]);

  const t = useLocale(locale);

  //提交修改
  const handleSubmit = () => {
    formRef.current.validate().then((values) => {
      setLoading(true);
      updateRole(values).then((res) => {
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
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
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

export default UpdatePage;
