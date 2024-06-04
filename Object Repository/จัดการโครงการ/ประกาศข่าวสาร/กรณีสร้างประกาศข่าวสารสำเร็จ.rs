<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>กรณีสร้างประกาศข่าวสารสำเร็จ</name>
   <tag></tag>
   <elementGuidId>7e7a3e6c-842d-465a-9408-e895a19c548a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;${uuid}\&quot;,\n  \&quot;image\&quot;: \&quot;https://residentialfamily-dev.centralpattana.co.th/dev/assets/upload/ads/905c3775-059d-4d0e-a918-5865093364dc.jpg\&quot;,\n  \&quot;project\&quot;: {\n    \&quot;id\&quot;: \&quot;a56b8623-1b44-4da8-9a19-d4479981c8fa\&quot;\n  },\n  \&quot;title\&quot;: {\n    \&quot;th\&quot;: \&quot;ทดสอบเพิ่มหัวข้อประกาศ\&quot;,\n    \&quot;en\&quot;: \&quot;Test Tiltle announcment\&quot;\n  },\n  \&quot;description\&quot;: {\n    \&quot;th\&quot;: \&quot;ทดสอบรายละเอียดแบบย่อ\&quot;,\n    \&quot;en\&quot;: \&quot;Test Detail\&quot;\n  },\n  \&quot;isHighlight\&quot;: null,\n  \&quot;publishedStartAt\&quot;: \&quot;2024-05-28T17:00:00.000Z\&quot;,\n  \&quot;publishedEndAt\&quot;: \&quot;2024-05-30T17:00:00.000Z\&quot;,\n  \&quot;content\&quot;: {\n    \&quot;th\&quot;: \&quot;\u003cp\u003eทดสอบรายละเอียด\u003c/p\u003e\&quot;,\n    \&quot;en\&quot;: \&quot;\u003cp\u003eTest Description\u003c/p\u003e\&quot;\n  },\n  \&quot;buttonType\&quot;: \&quot;url\&quot;,\n  \&quot;buttonName\&quot;: {\n    \&quot;th\&quot;: \&quot;ทดสอบเพิ่มชื่อปุ่ม\&quot;,\n    \&quot;en\&quot;: \&quot;Test Button name\&quot;\n  },\n  \&quot;buttonValue\&quot;: \&quot;Test button\&quot;,\n  \&quot;album\&quot;: [],\n  \&quot;tag\&quot;: \&quot;announcement\&quot;,\n  \&quot;publishedDateEnd\&quot;: null,\n  \&quot;highlightType\&quot;: null,\n  \&quot;highlightStartAt\&quot;: null,\n  \&quot;highlightEndAt\&quot;: null,\n  \&quot;active\&quot;: false,\n  \&quot;album_status\&quot;: null,\n  \&quot;userEdit\&quot;: \&quot;ภัคศุภาวงค์ จั่นแพ แก้ไขเมื่อ 14/08/64\&quot;,\n  \&quot;projectShow\&quot;: [],\n  \&quot;group_list\&quot;: null,\n  \&quot;ads_show\&quot;: null,\n  \&quot;notification\&quot;: false,\n  \&quot;notificationScheduleId\&quot;: null,\n  \&quot;project_about\&quot;: null\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1ee9019e-ab46-459c-aa22-9090f6bd76fe</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>0ac60c2f-965f-4d22-84c9-bfc3b521e535</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/project/a56b8623-1b44-4da8-9a19-d4479981c8fa/announcement</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>751617a1-c0a9-4202-9dd5-5bdf0749333d</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.uuid</defaultValue>
      <description></description>
      <id>051d3043-b943-4408-8cb6-65cc6b621d05</id>
      <masked>false</masked>
      <name>uuid</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
